#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket::http::{Status, ContentType, MediaType};
//use rocket::response::content;


#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old name {}!", age, name)
}

#[get("/dcap/edev/<id>",format = "application/sep+xml")]
fn edev(id: usize) -> Result<String, Status> {
    if id == 0 {
        let ret: String = format!("EndDevice {} exists!", id);
        return Ok(ret);
    }else if id == 1{
        let ret = r#"    <EndDevice xmlns="http://zigbee.org/sep" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="/dcap/edev/1" subscribable="0">
        <DERListLink href="/dcap/edev/1/der" all="1"/>
        <DeviceInformationLink href="/dcap/edev/1/di"/>
        <DeviceStatusLink href="/dcap/edev/1/dstat"/>
        <loadShedDeviceCategory>0200</loadShedDeviceCategory>
        <PowerStatusLink href="/dcap/edev/1/ps"/>
        <sFDI>111576577659</sFDI>
        <FunctionSetAssignmentsListLink href="/dcap/edev/1/fsa" all="1"/>
        <RegistrationLink href="/dcap/edev/1/reg"/>
    </EndDevice>
"#;
        return Ok(ret.to_string());
    }
    return Err(Status::NotFound);
}

#[post("/dcap/edev/<id>/di", format = "application/sep+xml", data = "<user_input>")]
fn edev_post(id: usize, user_input: String) -> Status {
    if id < 2 {
        println!("Post of id {} worked!
        {}", id, user_input);
        return Status::NoContent;
    }
    return Status::NotFound;
}

#[get("/dcap/edev/<id>/der/<der_id>/dera")]
fn dera(id: usize, der_id: usize) -> String {
    if id == 0 && der_id ==1 {
        format!("EndDevice dera {} exists!", id)
    }else{
        format!("EndDevice dera {} do not exist!", id)
    }
}

/*
#[get("/dcap2")]
fn just_fail() -> Status {
    Status::NotAcceptable
}
*/

#[launch]
fn rocket() -> _ {
    let custom = MediaType::new("application", "sep+xml");
    assert_eq!(custom.top(), "application");
    assert_eq!(custom.sub(), "sep+xml");
    let custom = ContentType::parse_flexible("application/sep+xml");
    assert_eq!(custom, Some(ContentType::new("application", "sep+xml")));
    rocket::build().mount("/", routes![hello, edev, edev_post, dera])
}

/*fn main() {
        rocket::ignite().mount("/", routes![hello]).launch();*/
    