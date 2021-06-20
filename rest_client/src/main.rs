//use reqwest::Client;
use std::fs::File;
use std::io::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents_cert = Vec::new();
    File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-4096/ca.pem")?
        .read_to_end(&mut contents_cert)?;
    // create a certificate
    let cert = reqwest::Certificate::from_pem(&contents_cert)?;

    let mut contents_id = Vec::new();
    //File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-4096/master.p12")?
    File::open("/home/gustavo/Documentos/git/IEEE-2030.5-Client/pki/RSA-4096/master.pem")?
        .read_to_end(&mut contents_id)?;
    // create a certificate
    //let identify = reqwest::Identity::from_pkcs12_der(&contents_id, "ieee2030")?;
    let identify = reqwest::Identity::from_pem(&contents_id)?;

    // get a client builder
    let client = reqwest::Client::builder()
        .add_root_certificate(cert)
        .identity(identify)
        //.danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let res = client.get("https://localhost:8443/dcap/edev/1")
        .header("Content-Type", "application/sep+xml")
        .send()
        .await?;

    println!("Status: {}", res.status());
    
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}
