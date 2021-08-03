//use reqwest::Client;
//use std::collections::HashMap;
use serde_json::{self, Number};
use tokio::task::block_in_place;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    country: String,
    #[serde(rename = "zip-code")]
    zipcode: Number
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    first_name: String,
    last_name: String,
    age: i32,
    weight: f32,
    address: Address,
    student: bool,
    skill: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    temperature: f32,
    humidity: i32,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents_cert = Vec::new();
    File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-2048/ca.pem")?
        .read_to_end(&mut contents_cert)?;
    // create a certificate
    let cert = reqwest::Certificate::from_pem(&contents_cert)?;

    let mut contents_id = Vec::new();
    //File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-4096/master.p12")?
    
    // Rusttls usa .pem. native-tls usa .p12
    //File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-2048/master.pem")?
    File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-2048/master.p12")?
        .read_to_end(&mut contents_id)?;
    // create a certificate
    //let identify = reqwest::Identity::from_pkcs12_der(&contents_id, "ieee2030")?;
    
    // Rusttls usa from_pem. native-tls usa from_pkcs12_der
    //let identify = reqwest::Identity::from_pem(&contents_id)?;

    let identify = reqwest::Identity::from_pkcs12_der(&contents_id, "")?;

    // get a client builder
    let client = reqwest::Client::builder()
        .add_root_certificate(cert)
        .identity(identify)
        .danger_accept_invalid_hostnames(true)  // para native-tls
        //.danger_accept_invalid_certs(true) // para rustls
        .build()?;
    
    /*
    //let res = client.get("https://localhost:8443/dcap/edev/1")
    let res = client.get("https://192.168.86.31/")
        //.header("Content-Type", "application/sep+xml2")
        .send()
        .await?;

    println!("Status: {}", res.status());
    
    let body = res.text().await?;
    println!("Body:\n\n{}", body);
     */


    
    let res = client.get("https://192.168.86.31/user")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:#?}", res);
     

    let res = client.get("https://192.168.86.31/user")
    .send()
    .await?
    .json::<Data>()
    .await?;

    println!("{:#?}", res);

    println!("First Name: {}",res.first_name);
    println!("Last Name: {}",res.last_name);
    println!("Contry: {}",res.address.country);
    println!("Zipcode: {}",res.address.zipcode);

    let res = client.get("https://192.168.86.31/weather")
    .send()
    .await?
    .json::<Weather>()
    .await?;

    println!("{:#?}", res);    
    

    Ok(())
}
