mod tags;

use std::collections::HashMap;
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
    osm_id: OsmID,
    kind: String,
    lon_lat: (f64, f64),
    name: Option<String>,
    brand: Option<String>,
    // TODO Add optional serde upstream
    opening_hours: Option<String>,
}

#[derive(Serialize, Deserialize)]
enum OsmID {
    Node(i64),
    Way(i64),
    Relation(i64),
}

impl Model {
    fn new_from_pbf(path: &str) -> Result<Self> {
        let mut model = Model {
            amenities: Vec::new(),
        };
        // Need these for ways/relations
        let mut node_positions = HashMap::new();
        let reader = ElementReader::from_path(path)?;
        reader.for_each(|element| match element {
            Element::Node(node) => {
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }
                let pt = (trim_f64(node.lon()), trim_f64(node.lat()));
                node_positions.insert(node.id(), pt);
                model.add_object(OsmID::Node(node.id()), tags, pt);
            }
            Element::DenseNode(node) => {
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }
                let pt = (trim_f64(node.lon()), trim_f64(node.lat()));
                node_positions.insert(node.id(), pt);
                model.add_object(OsmID::Node(node.id()), tags, pt);
            }
            Element::Way(way) => {
                let mut tags = Tags::new();
                for (k, v) in way.tags() {
                    tags.insert(k, v);
                }
                // TODO Centroid or something nicer
                model.add_object(
                    OsmID::Way(way.id()),
                    tags,
                    node_positions[&way.refs().next().unwrap()],
                );
            }
            Element::Relation(_) => {
                // TODO Handle. What about when they're large, or might be double-tagged?
                // https://www.openstreetmap.org/relation/14875126
            }
        })?;
        Ok(model)
    }

    fn add_object(&mut self, osm_id: OsmID, tags: Tags, lon_lat: (f64, f64)) {
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
                "parking_space",
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
                osm_id,
                kind,
                lon_lat,
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
            match amenity.osm_id {
                OsmID::Node(x) => feature.set_property("node_id", x),
                OsmID::Way(x) => feature.set_property("way_id", x),
                OsmID::Relation(x) => feature.set_property("relation_id", x),
            }
            feature.set_property("opening_hours", amenity.opening_hours.clone());
            writer.write_feature(&feature)?;
        }
        Ok(())
    }
}

fn trim_f64(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}
