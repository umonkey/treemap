/// Split items into maximal runs of visible items.
///
/// Hidden items act as dividers and are never included in the result. A run is
/// also flushed when the `panorama_id` changes. Empty runs from consecutive
/// hidden items are discarded.
pub fn group_visible<T, FId, FHid>(
    items: &[T],
    mut panorama_id: FId,
    mut is_hidden: FHid,
) -> Vec<Vec<&T>>
where
    FId: FnMut(&T) -> u64,
    FHid: FnMut(&T) -> bool,
{
    let mut groups = Vec::new();
    let mut current: Vec<&T> = Vec::new();
    let mut current_panorama_id: Option<u64> = None;

    for item in items {
        let id = panorama_id(item);
        let changed = current_panorama_id.is_some_and(|current_id| current_id != id);
        let hidden = is_hidden(item);

        if (hidden || changed) && !current.is_empty() {
            groups.push(std::mem::take(&mut current));
        }

        current_panorama_id = Some(id);

        if !hidden {
            current.push(item);
        }
    }

    if !current.is_empty() {
        groups.push(current);
    }

    groups
}
