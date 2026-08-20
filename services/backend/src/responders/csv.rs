use crate::domain::tree::Tree;
use crate::types::Error;
use crate::types::Result;
use actix_web::HttpResponse;
use csv::Writer;

fn format_timestamp(timestamp: u64) -> String {
    if timestamp == 0 {
        return "".to_string();
    }
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
        .unwrap_or_default()
}

pub fn trees_to_csv(trees: Vec<Tree>, filename: &str) -> Result<HttpResponse> {
    let mut wtr = Writer::from_writer(vec![]);

    wtr.write_record([
        "id",
        "lat",
        "lon",
        "state",
        "species",
        "height",
        "crown",
        "circumference",
        "added_at",
        "updated_at",
    ])
    .map_err(|e| Error::Config(e.to_string()))?;

    for tree in trees {
        wtr.write_record([
            format!("#{}", tree.id),
            format!("{:.7}", tree.lat),
            format!("{:.7}", tree.lon),
            tree.state,
            tree.species,
            tree.height.unwrap_or(0.0).to_string(),
            tree.diameter.unwrap_or(0.0).to_string(),
            tree.circumference.unwrap_or(0.0).to_string(),
            format_timestamp(tree.added_at),
            format_timestamp(tree.updated_at),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(0), "");
        assert_eq!(format_timestamp(1700000000), "2023-11-14T22:13:20Z");
    }

    #[tokio::test]
    async fn test_trees_to_csv() {
        let tree = Tree {
            id: 42,
            lat: 40.1792,
            lon: 44.5091,
            state: "healthy".to_string(),
            species: "Tilia cordata".to_string(),
            added_at: 1700000000,
            updated_at: 0,
            ..Default::default()
        };

        let response = trees_to_csv(vec![tree], "export").unwrap();
        assert_eq!(response.status(), actix_web::http::StatusCode::OK);

        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .unwrap();
        let csv_content = String::from_utf8(body.to_vec()).unwrap();

        assert!(csv_content.contains("id,lat,lon"));
        assert!(csv_content.contains(
            "#42,40.1792000,44.5091000,healthy,Tilia cordata,0,0,0,2023-11-14T22:13:20Z,"
        ));
    }
}
