use crate::domain::tree::Tree;
use crate::types::Error;
use crate::types::Result;
use actix_web::HttpResponse;
use csv::Writer;

pub fn trees_to_csv(trees: Vec<Tree>, filename: &str) -> Result<HttpResponse> {
    let mut wtr = Writer::from_writer(vec![]);

    wtr.write_record([
        "id", "lat", "lon", "state", "species", "crown", "girth", "height",
    ])
    .map_err(|e| Error::Config(e.to_string()))?;

    for tree in trees {
        wtr.write_record([
            tree.id.to_string(),
            format!("{:.7}", tree.lat),
            format!("{:.7}", tree.lon),
            tree.state,
            tree.species,
            tree.diameter.unwrap_or(0.0).to_string(),
            tree.circumference.unwrap_or(0.0).to_string(),
            tree.height.unwrap_or(0.0).to_string(),
        ])
        .map_err(|e| Error::Config(e.to_string()))?;
    }

    let data = wtr.into_inner().map_err(|e| Error::Config(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{filename}.csv\""),
        ))
        .body(data))
}
