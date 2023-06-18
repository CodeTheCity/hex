use crate::geojson::{parse_json_file,Coordinates};
use std::collections::HashMap;

mod hexjson;
mod geojson;

fn main() {
    let file_path = "geo.json";

    match parse_json_file(file_path) {
        Ok(geo) => println!("{:#?}", geo),
        Err(e) => println!("Failed to read or parse file: {}", e),
    }
}

fn average_coordinates(coords: &Vec<Coordinates>) -> Option<Coordinates> {
    if coords.is_empty() {
        return None;
    }
    
    let mut sum_longitude = 0.0;
    let mut sum_latitude = 0.0;
    for coord in coords {
        sum_longitude += coord.longitude;
        sum_latitude += coord.latitude;
    }
    
    Some(Coordinates {
        longitude: sum_longitude / (coords.len() as f64),
        latitude: sum_latitude / (coords.len() as f64),
    })
}

#[cfg(test)]
mod tests {
    use crate::hexjson::{Hex, HexJson, generate_hex_json};

    use super::*;

    #[test]
    fn test_convert_geojson_to_hexjson() {
        let file_path = "data/Manual_AberdeenshireIZHexMap.geojson";

        let geojson = parse_json_file(file_path).unwrap();
        let centroids = geojson.features.iter().map(|f| average_coordinates(&f.geometry.coordinates[0]).unwrap());
        let min_x = centroids.clone().map(|c|c.longitude).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        let max_x = centroids.clone().map(|c|c.longitude).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        let min_y = centroids.clone().map(|c|c.latitude).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        let max_y = centroids.clone().map(|c|c.latitude).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        println!("Bounds: {},{} : {},{}", min_x,min_y,max_x,max_y);
        let x_width = 30;
        let y_height = 30;
        let x_step = (max_x - min_x) / x_width as f64;
        let y_step = (max_y - min_y) / y_height as f64;

        let mut hexes = HashMap::new();
        for feature in geojson.features.iter() {
            println!("{:?}", feature);
            let coordinates = &feature.geometry.coordinates[0];
            let centroid = average_coordinates(coordinates).unwrap();
            println!("Centroid: {:?}", centroid);
            let hex = Hex {
                n: feature.properties.name.clone(),
                q: ((centroid.longitude - min_x) / x_step) as i32,
                r: ((centroid.latitude - min_y) / y_step) as i32,
                region: feature.properties.izcode.clone(), // FIXME: this should be the region!
                colour: "#2254F4".to_string(),
            };
            println!("{:?}", hex);
            hexes.insert(feature.properties.izcode.clone(), hex);
        }

        let hexjson = HexJson {
            layout: "odd-r".to_string(),
            hexes,
        };

        let filename = "tmp/output_hexjson.json";
        generate_hex_json(filename, &hexjson).expect("Failed to write HexJson to file");

    }
}
