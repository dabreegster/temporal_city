mod tags;

use std::io::{BufReader, BufWriter};

use anyhow::Result;
use fs_err::File;
use opening_hours::OpeningHours;
use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};

use crate::tags::Tags;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    let model = if args[1].ends_with(".osm.pbf") {
        let model = Model::new_from_pbf(&args[1])?;
        let writer = BufWriter::new(File::create("model.bin")?);
        bincode::serialize_into(writer, &model)?;
        model
    } else if args[1].ends_with(".bin") {
        bincode::deserialize_from(BufReader::new(File::open(&args[1])?))?
    } else {
        panic!("Gimme .bin or .osm.pbf");
    };

    println!("{} total", model.amenities.len());
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Model {
    amenities: Vec<Amenity>,
}

#[derive(Serialize, Deserialize)]
struct Amenity {
    kind: String,
    lon_lat: (f64, f64),
    name: Option<String>,
    brand: Option<String>,
    // TODO Add optional serde upstream
    opening_hours: Option<String>,
}

impl Model {
    fn new_from_pbf(path: &str) -> Result<Self> {
        let mut model = Model {
            amenities: Vec::new(),
        };
        let reader = ElementReader::from_path(path)?;
        reader.for_each(|element| match element {
            Element::Node(node) => {
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }
                model.handle_node(tags, node.lon(), node.lat());
            }
            Element::DenseNode(node) => {
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }
                model.handle_node(tags, node.lon(), node.lat());
            }
            _ => {}
        })?;
        Ok(model)
    }

    fn handle_node(&mut self, tags: Tags, lon: f64, lat: f64) {
        if let Some(kind) = tags.get("amenity") {
            self.amenities.push(Amenity {
                kind: kind.to_string(),
                lon_lat: (lon, lat),
                name: tags.get("name").cloned(),
                brand: tags.get("brand").cloned(),
                opening_hours: tags.get("opening_hours").cloned(),
            });
        }
    }
}
