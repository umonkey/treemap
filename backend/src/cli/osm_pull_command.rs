use crate::services::*;

pub async fn osm_pull_command() {
    let locator = Locator::new();

    let service = locator
        .get::<OsmReaderService>()
        .expect("Error creating OSM reader service.");

    service
        .pull_trees()
        .await
        .expect("Error pulling trees from OSM.");

    service
        .match_trees()
        .await
        .expect("Error matching OSM trees to local.");
}
