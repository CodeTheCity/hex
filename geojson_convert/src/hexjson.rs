use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json;
use serde_json::Error as SerdeError;
use std::fs;
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hex {
    pub n: String,
    pub q: i32,
    pub r: i32,
    pub region: String,
    pub colour: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HexJson {
    pub layout: String,
    pub hexes: HashMap<String, Hex>,
}

pub fn parse_hex_json(json: &str) -> Result<HexJson, SerdeError> {
    serde_json::from_str(json)
}

pub fn generate_hex_json(filename: &str, hexjson: &HexJson) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(hexjson)
        .expect("Failed to serialize HexJson object");
    fs::write(filename, json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_hex_json() {
        // read test data from file
        let json = fs::read_to_string("tests/test_hexjson.json")
            .expect("Unable to read test data");

        let parsed = parse_hex_json(&json).unwrap();

        assert_eq!(parsed.layout, "odd-r");
        assert!(parsed.hexes.contains_key("S12000005"));
        assert_eq!(parsed.hexes.get("S12000005").unwrap().n, "Clackmannanshire");
    }

    #[test]
    fn test_generate_hex_json() {
        let hex = Hex {
            n: "Clackmannanshire".to_string(),
            q: 2,
            r: 24,
            region: "S92000003".to_string(),
            colour: "#2254F4".to_string(),
        };

        let mut hexes = HashMap::new();
        hexes.insert("S12000005".to_string(), hex);

        let hexjson = HexJson {
            layout: "odd-r".to_string(),
            hexes,
        };

        let filename = "tmp/output_hexjson.json";
        generate_hex_json(filename, &hexjson).expect("Failed to write HexJson to file");
        // Here you might want to add some code to verify that the file was written correctly
    }
}