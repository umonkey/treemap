//! Proximity-based spatial clustering for duplicate tree detection.

use crate::domain::tree::{Bounds, TreeLocation, TreeState};
use std::collections::HashMap;

pub const DEFAULT_PROXIMITY_METERS: f64 = 1.0;

const METERS_PER_DEGREE_LAT: f64 = 111_320.0;

// Guard against division by zero near the poles.
const MIN_METERS_PER_DEGREE_LON: f64 = 1.0;

/// Returns the grid cell size in degrees for the given bounding box.
///
/// The cell size is chosen so that one cell spans roughly `proximity` meters
/// along the longitude axis at the latitude with the largest absolute value.
pub fn get_clustering_size(bbox: &Bounds, proximity: f64) -> f64 {
    let proximity = if proximity > 0.0 {
        proximity
    } else {
        DEFAULT_PROXIMITY_METERS
    };

    let lat = if bbox.n.abs() > bbox.s.abs() {
        bbox.n
    } else {
        bbox.s
    };

    let meters_per_degree_lon = (METERS_PER_DEGREE_LAT * lat.to_radians().cos())
        .abs()
        .max(MIN_METERS_PER_DEGREE_LON);

    proximity / meters_per_degree_lon.min(METERS_PER_DEGREE_LAT)
}

/// Buckets tree indices into a spatial grid keyed by cell coordinates.
pub fn clusterize(
    trees: &[TreeLocation],
    bbox: &Bounds,
    cell_size: f64,
) -> HashMap<(i64, i64), Vec<usize>> {
    let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();

    if cell_size <= 0.0 {
        return grid;
    }

    for (index, tree) in trees.iter().enumerate() {
        let x = ((tree.lon - bbox.w) / cell_size).floor() as i64;
        let y = ((tree.lat - bbox.s) / cell_size).floor() as i64;
        grid.entry((x, y)).or_default().push(index);
    }

    grid
}

/// Finds the closest target-state tree within `proximity` for every source-state tree.
///
/// Each source tree scans its own cell plus the eight adjacent cells. Ties are
/// broken deterministically by the lower target id.
pub fn find_candidates(
    trees: &[TreeLocation],
    grid: &HashMap<(i64, i64), Vec<usize>>,
    bbox: &Bounds,
    cell_size: f64,
    proximity: f64,
    source_states: &[TreeState],
    target_states: &[TreeState],
) -> Vec<(TreeLocation, TreeLocation)> {
    let mut candidates = Vec::new();

    if cell_size <= 0.0 || proximity <= 0.0 {
        return candidates;
    }

    let lat = if bbox.n.abs() > bbox.s.abs() {
        bbox.n
    } else {
        bbox.s
    };

    let meters_per_degree_lon = (METERS_PER_DEGREE_LAT * lat.to_radians().cos())
        .abs()
        .max(MIN_METERS_PER_DEGREE_LON);

    for source in trees.iter().filter(|t| source_states.contains(&t.state)) {
        let x = ((source.lon - bbox.w) / cell_size).floor() as i64;
        let y = ((source.lat - bbox.s) / cell_size).floor() as i64;

        let mut best: Option<(f64, &TreeLocation)> = None;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let Some(indices) = grid.get(&(x + dx, y + dy)) else {
                    continue;
                };

                for &index in indices {
                    let target = &trees[index];

                    if target.id == source.id || !target_states.contains(&target.state) {
                        continue;
                    }

                    let distance = distance_meters(source, target, meters_per_degree_lon);

                    if distance > proximity {
                        continue;
                    }

                    match best {
                        Some((best_distance, best_target))
                            if distance < best_distance
                                || (distance == best_distance && target.id < best_target.id) =>
                        {
                            best = Some((distance, target));
                        }
                        None => best = Some((distance, target)),
                        _ => {}
                    }
                }
            }
        }

        if let Some((_, target)) = best {
            candidates.push((source.clone(), target.clone()));
        }
    }

    candidates
}

/// Keeps only the trees whose state is in the `allowed` list.
pub fn filter_by_states(trees: Vec<TreeLocation>, allowed: &[TreeState]) -> Vec<TreeLocation> {
    trees
        .into_iter()
        .filter(|tree| allowed.contains(&tree.state))
        .collect()
}

/// Computes the bounding box of the given trees, or `None` when empty.
pub fn bbox_of(trees: &[TreeLocation]) -> Option<Bounds> {
    let first = trees.first()?;

    let mut n = first.lat;
    let mut s = first.lat;
    let mut e = first.lon;
    let mut w = first.lon;

    for tree in trees.iter().skip(1) {
        n = n.max(tree.lat);
        s = s.min(tree.lat);
        e = e.max(tree.lon);
        w = w.min(tree.lon);
    }

    Some(Bounds { n, e, s, w })
}

fn distance_meters(a: &TreeLocation, b: &TreeLocation, meters_per_degree_lon: f64) -> f64 {
    let dx = (a.lon - b.lon) * meters_per_degree_lon;
    let dy = (a.lat - b.lat) * METERS_PER_DEGREE_LAT;
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn location(id: u64, lat: f64, lon: f64, state: TreeState) -> TreeLocation {
        TreeLocation {
            id,
            lat,
            lon,
            state,
        }
    }

    #[test]
    fn test_get_clustering_size() {
        let equator = Bounds {
            n: 0.0,
            e: 0.0,
            s: 0.0,
            w: 0.0,
        };

        let size_equator = get_clustering_size(&equator, DEFAULT_PROXIMITY_METERS);
        assert!(size_equator > 0.0);
        assert!((size_equator - 1.0 / METERS_PER_DEGREE_LAT).abs() < 1e-12);

        let north = Bounds {
            n: 60.0,
            e: 0.0,
            s: 60.0,
            w: 0.0,
        };

        let size_north = get_clustering_size(&north, DEFAULT_PROXIMITY_METERS);
        assert!(size_north > size_equator);

        // Non-positive proximity falls back to the default.
        let fallback = get_clustering_size(&equator, 0.0);
        assert!((fallback - size_equator).abs() < 1e-12);
    }

    #[test]
    fn test_clusterize_groups_by_cell() {
        let bbox = Bounds {
            n: 1.0,
            e: 1.0,
            s: 0.0,
            w: 0.0,
        };

        let trees = vec![
            location(1, 0.1, 0.1, TreeState::Alive),
            location(2, 0.1, 0.1, TreeState::Alive),
            location(3, 0.9, 0.9, TreeState::Alive),
        ];

        let grid = clusterize(&trees, &bbox, 0.5);

        assert_eq!(grid.get(&(0, 0)).map(Vec::len), Some(2));
        assert_eq!(grid.get(&(1, 1)).map(Vec::len), Some(1));
    }

    #[test]
    fn test_find_candidates_across_adjacent_cells() {
        let bbox = Bounds {
            n: 40.001,
            e: 44.001,
            s: 40.0,
            w: 44.0,
        };

        let cell_size = get_clustering_size(&bbox, DEFAULT_PROXIMITY_METERS);

        // Two trees straddling the boundary between adjacent cells, but only
        // a fraction of a meter apart.
        let source = location(1, 40.0, 44.0 + cell_size * 0.99, TreeState::Gone);
        let target = location(2, 40.0, 44.0 + cell_size * 1.01, TreeState::Alive);
        let trees = vec![source, target];

        let grid = clusterize(&trees, &bbox, cell_size);
        assert_eq!(grid.len(), 2, "trees must land in different cells");

        let candidates = find_candidates(
            &trees,
            &grid,
            &bbox,
            cell_size,
            DEFAULT_PROXIMITY_METERS,
            &[TreeState::Gone],
            &[TreeState::Alive],
        );

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].0.id, 1);
        assert_eq!(candidates[0].1.id, 2);
    }

    #[test]
    fn test_find_candidates_beyond_proximity() {
        let bbox = Bounds {
            n: 40.001,
            e: 44.001,
            s: 40.0,
            w: 44.0,
        };

        let cell_size = get_clustering_size(&bbox, DEFAULT_PROXIMITY_METERS);

        // Two trees roughly two meters apart, beyond the one meter proximity.
        let source = location(1, 40.0, 44.0, TreeState::Gone);
        let target = location(
            2,
            40.0 + 2.0 / METERS_PER_DEGREE_LAT,
            44.0,
            TreeState::Alive,
        );
        let trees = vec![source, target];

        let grid = clusterize(&trees, &bbox, cell_size);

        let candidates = find_candidates(
            &trees,
            &grid,
            &bbox,
            cell_size,
            DEFAULT_PROXIMITY_METERS,
            &[TreeState::Gone],
            &[TreeState::Alive],
        );

        assert!(candidates.is_empty());
    }

    #[test]
    fn test_find_candidates_picks_closest_target() {
        let bbox = Bounds {
            n: 40.001,
            e: 44.001,
            s: 40.0,
            w: 44.0,
        };

        let cell_size = get_clustering_size(&bbox, DEFAULT_PROXIMITY_METERS);

        let source = location(1, 40.0, 44.0, TreeState::Stump);
        let far = location(
            2,
            40.0,
            44.0 + 0.5 / METERS_PER_DEGREE_LAT,
            TreeState::Alive,
        );
        let near = location(
            3,
            40.0,
            44.0 + 0.1 / METERS_PER_DEGREE_LAT,
            TreeState::Alive,
        );
        let trees = vec![source, far, near];

        let grid = clusterize(&trees, &bbox, cell_size);

        let candidates = find_candidates(
            &trees,
            &grid,
            &bbox,
            cell_size,
            DEFAULT_PROXIMITY_METERS,
            &[TreeState::Stump],
            &[TreeState::Alive],
        );

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].1.id, 3);
    }

    #[test]
    fn test_filter_by_states() {
        let trees = vec![
            location(1, 0.0, 0.0, TreeState::Alive),
            location(2, 0.0, 0.0, TreeState::Dead),
            location(3, 0.0, 0.0, TreeState::Gone),
        ];

        let filtered = filter_by_states(trees, &[TreeState::Alive, TreeState::Gone]);
        let ids: Vec<u64> = filtered.iter().map(|t| t.id).collect();

        assert_eq!(ids, vec![1, 3]);
    }

    #[test]
    fn test_bbox_of() {
        assert!(bbox_of(&[]).is_none());

        let trees = vec![
            location(1, 40.0, 44.0, TreeState::Alive),
            location(2, 41.0, 45.0, TreeState::Alive),
            location(3, 39.0, 43.0, TreeState::Alive),
        ];

        let bbox = bbox_of(&trees).expect("bbox must exist");
        assert_eq!(bbox.n, 41.0);
        assert_eq!(bbox.s, 39.0);
        assert_eq!(bbox.e, 45.0);
        assert_eq!(bbox.w, 43.0);
    }
}
