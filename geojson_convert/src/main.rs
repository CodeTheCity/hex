use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GeoJson {
    #[serde(rename = "type")]
    geo_type: String,
    name: String,
    crs: CRS,
    features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CRS {
    #[serde(rename = "type")]
    crs_type: String,
    properties: CRSProperties,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CRSProperties {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Feature {
    #[serde(rename = "type")]
    feature_type: String,
    properties: FeatureProperties,
    geometry: Geometry,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FeatureProperties {
    #[serde(rename = "IZcode")]
    izcode: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Geometry {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<Vec<Vec<f64>>>,
}

fn parse_json_file(file_path: &str) -> Result<GeoJson, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let geo_json: GeoJson = serde_json::from_str(&data)?;
    Ok(geo_json)
}

fn main() {
    let file_path = "geo.json";

    match parse_json_file(file_path) {
        Ok(geo) => println!("{:#?}", geo),
        Err(e) => println!("Failed to read or parse file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_file() {
        let file_path = "tests/test_geojson.json";

        let expected = GeoJson {
            geo_type: "FeatureCollection".to_string(),
            name: "AberdeenshireIZs2".to_string(),
            crs: CRS {
                crs_type: "name".to_string(),
                properties: CRSProperties {
                    name: "urn:ogc:def:crs:OGC:1.3:CRS84".to_string(),
                },
            },
            features: vec![Feature {
                feature_type: "Feature".to_string(),
                properties: FeatureProperties {
                    izcode: "S02001285".to_string(),
                    name: "East Cairngorms".to_string(),
                },
                geometry: Geometry {
                    geometry_type: "Polygon".to_string(),
                    coordinates: vec![vec![
                        vec![0.02, -0.05962],
                        vec![0.02, -0.06346],
                        vec![0.01667, -0.06539],
                        vec![0.01333, -0.06346],
                        vec![0.01333, -0.05962],
                        vec![0.01667, -0.05769],
                        vec![0.02, -0.05962],
                        vec![0.02, -0.05962],
                    ]],
                },
            }],
        };

        assert_eq!(parse_json_file(file_path).unwrap(), expected);
    }
}
