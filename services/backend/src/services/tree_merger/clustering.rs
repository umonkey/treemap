//! Proximity-based spatial clustering for duplicate tree detection.

use crate::domain::tree::{TreeLocation, TreeState};
use std::collections::HashMap;

pub const DEFAULT_PROXIMITY_METERS: f64 = 1.0;

const METERS_PER_DEGREE_LAT: f64 = 111_320.0;

// Guard against division by zero near the poles.
const MIN_METERS_PER_DEGREE_LON: f64 = 1.0;

/// Trees indexed into a spatial grid, together with the parameters used to
/// build it.
pub struct ClusteredData {
    pub trees: Vec<TreeLocation>,
    pub grid: HashMap<(i64, i64), Vec<usize>>,
    pub cell_size: f64,
    pub meters_per_degree_lon: f64,
}

/// Returns the grid cell size in degrees for a dataset centered at `center_lat`.
pub fn get_clustering_size(center_lat: f64, proximity: f64) -> f64 {
    let proximity = if proximity > 0.0 {
        proximity
    } else {
        DEFAULT_PROXIMITY_METERS
    };

    let meters_per_degree_lon = meters_per_degree_lon(center_lat);

    proximity / meters_per_degree_lon.min(METERS_PER_DEGREE_LAT)
}

/// Buckets trees into a spatial grid.
///
/// The center of the dataset is computed internally to derive the cell size,
/// so callers do not need to pass a bounding box around.
pub fn clusterize(trees: Vec<TreeLocation>, proximity: f64) -> ClusteredData {
    let center_lat = center_lat(&trees);
    let cell_size = get_clustering_size(center_lat, proximity);
    let meters_per_degree_lon = meters_per_degree_lon(center_lat);

    let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();

    if cell_size > 0.0 {
        for (index, tree) in trees.iter().enumerate() {
            grid.entry(cell_of(tree, cell_size))
                .or_default()
                .push(index);
        }
    }

    ClusteredData {
        trees,
        grid,
        cell_size,
        meters_per_degree_lon,
    }
}

/// Finds the closest target-state tree within `proximity` for every source-state tree.
///
/// Each source tree scans its own cell plus the eight adjacent cells. Ties are
/// broken deterministically by the lower target id.
pub fn find_candidates(
    clustered: &ClusteredData,
    proximity: f64,
    source_states: &[TreeState],
    target_states: &[TreeState],
) -> Vec<(TreeLocation, TreeLocation)> {
    let mut candidates = Vec::new();

    if clustered.cell_size <= 0.0 || proximity <= 0.0 {
        return candidates;
    }

    for source in clustered
        .trees
        .iter()
        .filter(|t| source_states.contains(&t.state))
    {
        let (x, y) = cell_of(source, clustered.cell_size);
        let mut best: Option<(f64, &TreeLocation)> = None;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let Some(indices) = clustered.grid.get(&(x + dx, y + dy)) else {
                    continue;
                };

                for &index in indices {
                    let target = &clustered.trees[index];

                    if target.id == source.id || !target_states.contains(&target.state) {
                        continue;
                    }

                    let distance = distance_meters(source, target, clustered.meters_per_degree_lon);

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

fn center_lat(trees: &[TreeLocation]) -> f64 {
    let Some(first) = trees.first() else {
        return 0.0;
    };

    let mut min = first.lat;
    let mut max = first.lat;

    for tree in trees.iter().skip(1) {
        min = min.min(tree.lat);
        max = max.max(tree.lat);
    }

    (min + max) / 2.0
}

fn meters_per_degree_lon(lat: f64) -> f64 {
    (METERS_PER_DEGREE_LAT * lat.to_radians().cos())
        .abs()
        .max(MIN_METERS_PER_DEGREE_LON)
}

fn cell_of(tree: &TreeLocation, cell_size: f64) -> (i64, i64) {
    let x = (tree.lon / cell_size).floor() as i64;
    let y = (tree.lat / cell_size).floor() as i64;
    (x, y)
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
        let size_equator = get_clustering_size(0.0, DEFAULT_PROXIMITY_METERS);
        assert!(size_equator > 0.0);
        assert!((size_equator - 1.0 / METERS_PER_DEGREE_LAT).abs() < 1e-12);

        let size_north = get_clustering_size(60.0, DEFAULT_PROXIMITY_METERS);
        assert!(size_north > size_equator);

        // Non-positive proximity falls back to the default.
        let fallback = get_clustering_size(0.0, 0.0);
        assert!((fallback - size_equator).abs() < 1e-12);
    }

    #[test]
    fn test_clusterize_groups_by_cell() {
        let trees = vec![
            location(1, 0.1, 0.1, TreeState::Alive),
            location(2, 0.1, 0.1, TreeState::Alive),
            location(3, 0.9, 0.9, TreeState::Alive),
        ];

        let clustered = clusterize(trees, DEFAULT_PROXIMITY_METERS);

        assert_eq!(clustered.grid.values().map(Vec::len).sum::<usize>(), 3);
        assert_eq!(clustered.grid.len(), 2);
    }

    #[test]
    fn test_find_candidates_across_adjacent_cells() {
        let cell_size = get_clustering_size(40.0, DEFAULT_PROXIMITY_METERS);

        // Place the trees on either side of an actual cell boundary.
        let boundary = (44.0 / cell_size).floor() * cell_size;

        let source = location(1, 40.0, boundary - cell_size * 0.01, TreeState::Gone);
        let target = location(2, 40.0, boundary + cell_size * 0.01, TreeState::Alive);
        let trees = vec![source, target];

        let clustered = clusterize(trees, DEFAULT_PROXIMITY_METERS);
        assert_eq!(
            clustered.grid.len(),
            2,
            "trees must land in different cells"
        );

        let candidates = find_candidates(
            &clustered,
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
        // Two trees roughly two meters apart, beyond the one meter proximity.
        let source = location(1, 40.0, 44.0, TreeState::Gone);
        let target = location(
            2,
            40.0 + 2.0 / METERS_PER_DEGREE_LAT,
            44.0,
            TreeState::Alive,
        );
        let trees = vec![source, target];

        let clustered = clusterize(trees, DEFAULT_PROXIMITY_METERS);

        let candidates = find_candidates(
            &clustered,
            DEFAULT_PROXIMITY_METERS,
            &[TreeState::Gone],
            &[TreeState::Alive],
        );

        assert!(candidates.is_empty());
    }

    #[test]
    fn test_find_candidates_picks_closest_target() {
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

        let clustered = clusterize(trees, DEFAULT_PROXIMITY_METERS);

        let candidates = find_candidates(
            &clustered,
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
    fn test_center_lat() {
        assert_eq!(center_lat(&[]), 0.0);

        let trees = vec![
            location(1, 40.0, 44.0, TreeState::Alive),
            location(2, 41.0, 45.0, TreeState::Alive),
            location(3, 39.0, 43.0, TreeState::Alive),
        ];

        assert_eq!(center_lat(&trees), 40.0);
    }
}
