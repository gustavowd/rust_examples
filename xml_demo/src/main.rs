extern crate serde;
extern crate serde_xml_rs;

#[macro_use]
extern crate serde_derive;

use serde_xml_rs::{from_str, to_string};
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,
    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
}

fn main() {
    let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
    let should_be = Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    };

    let item: Item = from_str(src).unwrap();
    assert_eq!(item, should_be);
    println!("Primeiro valor: {}", should_be.name);
    println!("Primeiro valor: {}", should_be.source);

    let reserialized_item = to_string(&item).unwrap();
    assert_eq!(src, reserialized_item);


    let s = "project.xml";
    let f = File::open(s).expect(&format!("Cannot open file {}", s));
    let r = BufReader::new(f);
    let prj: Project = serde_xml_rs::de::from_reader(r).unwrap();
    println!("{:?}", prj);
}
