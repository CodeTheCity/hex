use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GeoJson {
    #[serde(rename = "type")]
    geo_type: String,
    name: String,
    crs: CRS,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CRS {
    #[serde(rename = "type")]
    crs_type: String,
    properties: CRSProperties,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CRSProperties {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Feature {
    #[serde(rename = "type")]
    feature_type: String,
    pub properties: FeatureProperties,
    pub geometry: Geometry,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FeatureProperties {
    #[serde(rename = "IZcode")]
    pub izcode: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Geometry {
    #[serde(rename = "type")]
    geometry_type: String,
    pub coordinates: Vec<Vec<Coordinates>>,
}

pub fn parse_json_file(file_path: &str) -> Result<GeoJson, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let geo_json: GeoJson = serde_json::from_str(&data)?;
    Ok(geo_json)
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
                        Coordinates{longitude: 0.02, latitude: -0.05962},
                        Coordinates{longitude: 0.02, latitude: -0.06346},
                        Coordinates{longitude: 0.01667, latitude: -0.06539},
                        Coordinates{longitude: 0.01333, latitude: -0.06346},
                        Coordinates{longitude: 0.01333, latitude: -0.05962},
                        Coordinates{longitude: 0.01667, latitude: -0.05769},
                        Coordinates{longitude: 0.02, latitude: -0.05962},
                        Coordinates{longitude: 0.02, latitude: -0.05962},
                    ]],
                },
            }],
        };

        assert_eq!(parse_json_file(file_path).unwrap(), expected);
    }
}
