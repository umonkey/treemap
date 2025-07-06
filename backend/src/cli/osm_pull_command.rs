use crate::services::*;

pub async fn osm_pull_command() {
    let locator = Locator::new();

    let service = locator
        .get::<OsmReaderService>()
        .expect("Error creating OSM reader service.");

    // Pull new tree nodes, put them in the osm_trees table.
    service
        .pull_trees()
        .await
        .expect("Error pulling trees from OSM.");

    // Match new OSM nodes to local trees using coordinates.
    service
        .match_trees()
        .await
        .expect("Error matching OSM trees to local.");
}
