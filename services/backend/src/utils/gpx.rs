use crate::types::*;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
pub struct GpxPoint {
    pub lat: f64,
    pub lon: f64,
    pub time: Option<String>,
}

pub fn parse_gpx(data: &[u8]) -> Result<Vec<GpxPoint>> {
    let parser = EventReader::new(data);
    let mut points = Vec::new();
    let mut current_lat: Option<f64> = None;
    let mut current_lon: Option<f64> = None;
    let mut current_time: Option<String> = None;
    let mut in_trkpt = false;
    let mut in_time = false;
    let mut time_buffer = String::new();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let local_name = name.local_name.to_lowercase();
                if local_name == "trkpt" || local_name == "rtept" || local_name == "wpt" {
                    in_trkpt = true;
                    current_lat = None;
                    current_lon = None;
                    current_time = None;
                    for attr in attributes {
                        let attr_name = attr.name.local_name.to_lowercase();
                        if attr_name == "lat" {
                            current_lat = attr.value.parse().ok();
                        } else if attr_name == "lon" || attr_name == "lng" {
                            current_lon = attr.value.parse().ok();
                        }
                    }
                } else if in_trkpt && local_name == "time" {
                    in_time = true;
                    time_buffer.clear();
                }
            }
            Ok(XmlEvent::Characters(text)) if in_time => {
                time_buffer.push_str(&text);
            }
            Ok(XmlEvent::EndElement { name }) => {
                let local_name = name.local_name.to_lowercase();
                if in_trkpt && local_name == "time" {
                    in_time = false;
                    current_time = Some(time_buffer.trim().to_string());
                } else if local_name == "trkpt" || local_name == "rtept" || local_name == "wpt" {
                    if let (Some(lat), Some(lon)) = (current_lat, current_lon) {
                        points.push(GpxPoint {
                            lat,
                            lon,
                            time: current_time.filter(|t| !t.is_empty()),
                        });
                    }
                    in_trkpt = false;
                    current_lat = None;
                    current_lon = None;
                    current_time = None;
                }
            }
            Err(e) => {
                return Err(Error::BadRequestMessage(format!(
                    "Failed to parse GPX: {e}"
                )));
            }
            _ => {}
        }
    }

    Ok(points)
}
