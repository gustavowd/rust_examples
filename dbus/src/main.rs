use dbus;
use dbus::{blocking::Connection, arg};
use dbus::Message;
use std::time::Duration;
//use std::thread;


pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

fn print_refarg(value: &dyn arg::RefArg) {
    // We don't know what type the value is. We'll try a few and fall back to
    // debug printing if the value is more complex than that.
    if let Some(s) = value.as_str() { println!("{}", s); }
    else if let Some(i) = value.as_i64() { println!("{}", i); }
    else if let Some(f) = value.as_f64() { println!("{}", f); }
    else { println!("{:?}", value); }
}

fn test(){
    // First open up a connection to the session bus.
    let conn = Connection::new_session().unwrap();

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("br.wdbus.Example","/br/wdbus/Example/One", Duration::from_millis(5000));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

    let metadata_signal2: arg::PropMap = proxy.get_all("br.wdbus.Example.HelloWorld").unwrap();
    //let metadata_signal2: arg::PropMap;
    //match proxy.get_all("br.wdbus.Example.HelloWorld"){
    //    Ok(value) => metadata_signal2 = value, // if Ok(255), set x to 255
    //    Err(e) => println!("Erro: {}", e),
    //}

    // We now iterate over the hashmap.
    for (key, value) in metadata_signal2.iter() {
        if key.eq("NumberOfGreetings"){
            print!("  {}: ", key);
            print_refarg(&value);
        }
    }
}

fn mult(x: i32, y: i32) -> i32 {
    let ret = x * y;
    return ret;
}

/*
fn function_with_error() -> Result<u64, String> {
    //if error happens
    if true {
        return Err("some message".to_string());
    }

    // else, return valid output
    Ok(255)
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("br.wdbus.Example","/br/wdbus/Example/One", Duration::from_millis(5000));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;


    let (s,): (String,) = proxy.method_call("br.wdbus.Example.HelloWorld", "Hello", ("Rust test",))?;
    println!("{}", s);

    //let greetings: arg::Variant<Box<dyn arg::RefArg + 'static>>  = proxy.get("br.wdbus.Example.HelloWorld", "NumberOfGreetings")?;
    let greetings: &dyn arg::RefArg = &(proxy.get("br.wdbus.Example.HelloWorld", "NumberOfGreetings") as Result<Box<dyn arg::RefArg + 'static>, dbus::Error>)?;
    println!("Propriedade NumberOfGreetings da interface br.wdbus.Example.HelloWorld");
    print_refarg(&greetings);
    
    let excited: &dyn arg::RefArg = &(proxy.get("br.wdbus.Example.HelloWorld", "Excited") as Result<Box<dyn arg::RefArg + 'static>, dbus::Error>)?;
    println!("Propriedade Excited da interface br.wdbus.Example.HelloWorld");
    print_refarg(&excited);

    let metadata: arg::PropMap = proxy.get_all("br.wdbus.Example.HelloWorld")?;
    
    println!("Propriedades da interface br.wdbus.Example.HelloWorld");
    // We now iterate over the hashmap.
    for (key, value) in metadata.iter() {
        print!("  {}: ", key);
        print_refarg(&value);
    }


    let (result,): (i32,) = proxy.method_call("br.wdbus.Example.HelloWorld", "Sum", (20,5,))?;
    println!("Resultado da soma: {}", result);

    let metadata2: arg::PropMap = proxy.get_all("br.wdbus.Example.DataInterface")?;
    
    println!("Propriedades da interface br.wdbus.Example.DataInterface");
    // We now iterate over the hashmap.
    for (key, value) in metadata2.iter() {
        print!("  {}: ", key);
        print_refarg(&value);
    }
    
    // Let's start listening to signals.
    let _id = proxy.match_signal(|h: OrgFreedesktopDBusPropertiesPropertiesChanged, _: &Connection, _: &Message| {
        println!("PropertiesChanged signal happened from interface:{}", h.interface_name);
        println!("Properties Changed");
        // We now iterate over the hashmap.
        for (key, value) in h.changed_properties.iter() {
            print!("  {}: ", key);
            print_refarg(&value);
        }
        if h.interface_name.eq("br.wdbus.Example.HelloWorld"){
            // Call function to search for greetings
            println!("Call function to search for greetings");
            test();
        }
        true
    });

    println!("Teste de função multiplica 6 por 7. Resultado = {}", mult(6,7));

    // Listen to incoming signals forever.
    loop { conn.process(Duration::from_millis(1000))?; }

    //Ok(())
}


