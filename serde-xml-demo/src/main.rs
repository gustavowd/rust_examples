#[macro_use]
extern crate serde_derive;

extern crate log;

extern crate serde;
extern crate serde_xml_rs;

use std::fmt::Debug;

use std::fs::File;
use serde_xml_rs::{from_reader, from_str};

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
    libraries: Libraries
}

#[derive(Deserialize, Debug)]
struct Libraries {
    #[serde(rename = "library")]
    libraries: Vec<Library>,
}


#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Library {
    groupId: String,
    artifactId: String,
    version: String,
}


#[derive(Deserialize, Debug)]
struct DeviceInformation {
    //href: String,
    #[serde(rename = "lFDI")]
    lfdi: String,
    #[serde(rename = "mfDate")]
    mfdate: u64,
    #[serde(rename = "mfHwVer")]
    mfhwver: String,
    #[serde(rename = "mfID")]
    mfid: u64,
    #[serde(rename = "mfInfo")]
    mfinfo: String,
    #[serde(rename = "mfModel")]
    mfmodel: String,
    #[serde(rename = "mfSerNum")]
    mfsernum: u64,
    #[serde(rename = "primaryPower")]
    primarypower: u64,
    #[serde(rename = "secondaryPower")]
    secondarypower: u64,
    #[serde(rename = "swActTime")]
    swacttime: u64,
    #[serde(rename = "swVer")]
    swver: String,
}

fn main() {
    let file = File::open("target/debug/sample.xml").unwrap();
    let project: Project = from_reader(file).unwrap();
    println!("{:?}", project);

    let s = r#"<project name="project-name">
    <libraries>
        <library groupId="org.example" artifactId="&lt;name&gt;" version="0.1"/>
        <library groupId="com.example" artifactId="&quot;cool-lib&amp;" version="999"/>
    </libraries>
</project>"#;

    let project2: Project = from_str(s).unwrap();
    println!("{:?}", project2);

    let xml = r#"<DeviceInformation xmlns="urn:ieee:std:2030.5:ns" href="/edev/0/di">
                        <functionsImplemented>0145</functionsImplemented>
                        <lFDI>5509D69F8B353595206AD71B47E27906318EA367</lFDI>
                        <mfDate>1388566800</mfDate>
                        <mfHwVer>MF-HW: 1.0.0</mfHwVer>
                        <mfID>37250</mfID>
                        <mfInfo>Mf Information</mfInfo>
                        <mfModel>Mf Model</mfModel>
                        <mfSerNum>1234567890</mfSerNum>
                        <primaryPower>2</primaryPower>
                        <secondaryPower>0</secondaryPower>
                        <swActTime>1416107035</swActTime>
                        <swVer>9bc8e7b_modified</swVer>
                        </DeviceInformation>"#;

    let deviceinformation: DeviceInformation = from_str(xml).unwrap();
    println!("{:?}", deviceinformation);
}


/*
#[macro_use]
extern crate serde_derive;

extern crate log;

extern crate serde;
extern crate serde_xml_rs;

use std::fmt::Debug;

use serde_xml_rs::{from_str};
use serde::{de, ser};


#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct Graphml {
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
}

fn test_parse_ok<'de, 'a, T>(errors: &[(&'a str, T)])
where
    T: PartialEq + Debug + ser::Serialize + de::Deserialize<'de>,
{
    for &(s, ref value) in errors {
        println!("{}", s);
        let v: T = from_str(s).unwrap();
        assert_eq!(v, *value);

        // // Make sure we can deserialize into an `Element`.
        // let xml_value: Element = from_str(s).unwrap();

        // // Make sure we can deserialize from an `Element`.
        // let v: T = from_value(xml_value.clone()).unwrap();
        // assert_eq!(v, *value);
    }
}



fn main() {
    let s = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <graphml xmlns="http://graphml.graphdrawing.org/xmlns"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns
        http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">
    </graphml>"#;

    test_parse_ok(&[
        (
            s,
            Graphml {
                schema_location: "xsi:schemaLocation=http://graphml.graphdrawing.org/xmlns
        http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd".to_string(),
            },
        ),
    ]);
}
*/