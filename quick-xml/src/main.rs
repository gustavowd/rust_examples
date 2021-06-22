/*
extern crate quick_xml;

fn main() {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    /*let xml = "<tag1>text1</tag1><tag1>text2</tag1>\
               <tag1>text3</tag1><tag1><tag2>text4</tag2></tag1>";*/
    
    let xml = r#"<DeviceInformation xmlns="http://zigbee.org/sep" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
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

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"functionsImplemented" => {
                txt.push(
                    reader
                        .read_text(b"functionsImplemented", &mut Vec::new())
                        .expect("Cannot decode text value"),
                );
                println!("{:?}", txt);
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }
}
*/

/*
use quick_xml::Reader;
use quick_xml::events::Event;

fn main() {
    let xml = r#"<tag1 att1 = "test">
                <tag2><!--Test comment-->Test</tag2>
                <tag2>
                    Test 2
                </tag2>
            </tag1>"#;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
        println!("Count: {}\n", count);
    }
}
*/

extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{from_str};

#[derive(Debug, Deserialize, PartialEq)]
struct Link {
    rel: String,
    href: String,
    sizes: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Fr,
    De,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Head {
    title: String,
    #[serde(rename = "link", default)]
    links: Vec<Link>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Script {
    src: String,
    integrity: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Body {
    #[serde(rename = "script", default)]
    scripts: Vec<Script>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Html {
    lang: Option<String>,
    head: Head,
    body: Body,
}

fn main(){
    let xml = "<!DOCTYPE html>
        <html lang=\"en\">
            <head>
                <title>crates.io: Rust Package Registry</title>
                <link rel=\"stylesheet\" href=\"/assets/vendor-8d023d47762d5431764f589a6012123e.css\" integrity=\"sha256-EoB7fsYkdS7BZba47+C/9D7yxwPZojsE4pO7RIuUXdE= sha512-/SzGQGR0yj5AG6YPehZB3b6MjpnuNCTOGREQTStETobVRrpYPZKneJwcL/14B8ufcvobJGFDvnTKdcDDxbh6/A==\" >
                <link rel=\"stylesheet\" href=\"/assets/cargo-cedb8082b232ce89dd449d869fb54b98.css\" integrity=\"sha256-S9K9jZr6nSyYicYad3JdiTKrvsstXZrvYqmLUX9i3tc= sha512-CDGjy3xeyiqBgUMa+GelihW394pqAARXwsU+HIiOotlnp1sLBVgO6v2ZszL0arwKU8CpvL9wHyLYBIdfX92YbQ==\" >
                <link rel=\"shortcut icon\" href=\"/favicon.ico\" type=\"image/x-icon\">
                <link rel=\"icon\" href=\"/cargo-835dd6a18132048a52ac569f2615b59d.png\" type=\"image/png\">
                <link rel=\"search\" href=\"/opensearch.xml\" type=\"application/opensearchdescription+xml\" title=\"Cargo\">
            </head>
            <body>
                <script src=\"/assets/vendor-bfe89101b20262535de5a5ccdc276965.js\" integrity=\"sha256-U12Xuwhz1bhJXWyFW/hRr+Wa8B6FFDheTowik5VLkbw= sha512-J/cUUuUN55TrdG8P6Zk3/slI0nTgzYb8pOQlrXfaLgzr9aEumr9D1EzmFyLy1nrhaDGpRN1T8EQrU21Jl81pJQ==\" ></script>
                <script src=\"/assets/cargo-4023b68501b7b3e17b2bb31f50f5eeea.js\" integrity=\"sha256-9atimKc1KC6HMJF/B07lP3Cjtgr2tmET8Vau0Re5mVI= sha512-XJyBDQU4wtA1aPyPXaFzTE5Wh/mYJwkKHqZ/Fn4p/ezgdKzSCFu6FYn81raBCnCBNsihfhrkb88uF6H5VraHMA==\" ></script>
            </body>
        </html>";
    let html: Html = from_str(xml).unwrap();  
    println!("{:?}",html.head.title);  
}