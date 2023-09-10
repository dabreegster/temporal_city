mod tags;

use std::io::{BufReader, BufWriter};

use anyhow::Result;
use fs_err::File;
use geojson::{Feature, FeatureWriter, Geometry, Value};
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
    model.write_gj("out.geojson")?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Model {
    amenities: Vec<Amenity>,
}

#[derive(Serialize, Deserialize)]
struct Amenity {
    node_id: i64,
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
                model.handle_node(node.id(), tags, node.lon(), node.lat());
            }
            Element::DenseNode(node) => {
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }
                model.handle_node(node.id, tags, node.lon(), node.lat());
            }
            _ => {}
        })?;
        Ok(model)
    }

    fn handle_node(&mut self, node_id: i64, tags: Tags, lon: f64, lat: f64) {
        // Only want places people spend time
        // TODO Allowlist might be easier
        if tags.is_any(
            "amenity",
            vec![
                "bicycle_parking",
                "bench",
                "post_box",
                "waste_basket",
                "telephone",
                "atm",
                "recycling",
                "bicycle_rental",
                "motorcycle_parking",
                "charging_station",
                "toilets",
                "parking",
                "car_sharing",
                "vending_machine",
                "parking_entrance",
                "waste_disposal",
                "housing_office",
                "bicycle_repair_station",
                "grit_bin",
                "dog_litter_box",
                "parking_meter",
                "drinking_water",
                "fuel",
                "parcel_locker",
                "taxi",
                "car_rental",
                "public_bookcase",
                "car_wash",
            ],
        ) {
            return;
        }

        if let Some(kind) = tags.get("amenity") {
            let kind = if let Some(cuisine) = tags.get("cuisine") {
                format!("{kind} ({cuisine})")
            } else {
                kind.to_string()
            };

            self.amenities.push(Amenity {
                node_id,
                kind,
                lon_lat: (trim_f64(lon), trim_f64(lat)),
                name: tags.get("name").cloned(),
                brand: tags.get("brand").cloned(),
                opening_hours: tags.get("opening_hours").cloned(),
            });
        }
    }

    fn write_gj(&self, path: &str) -> Result<()> {
        let mut writer = FeatureWriter::from_writer(BufWriter::new(File::create(path)?));
        for amenity in &self.amenities {
            let mut feature = Feature::from(Geometry::new(Value::Point(vec![
                amenity.lon_lat.0,
                amenity.lon_lat.1,
            ])));
            feature.set_property("kind", amenity.kind.clone());
            feature.set_property("name", amenity.name.clone());
            feature.set_property("brand", amenity.brand.clone());
            feature.set_property("node_id", amenity.node_id);
            feature.set_property("opening_hours", amenity.opening_hours.clone());
            writer.write_feature(&feature)?;
        }
        Ok(())
    }
}

fn trim_f64(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}
