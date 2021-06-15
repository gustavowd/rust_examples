/*
extern crate env_logger;
extern crate mdns;

pub fn main() {
    env_logger::init();

    let responder = mdns::Responder::new().unwrap();

    loop {
        let _svc = responder.register(
            //"_googlecast._tcp".to_owned(),
            "tm._sub._smartenergy._tcp".to_owned(),
            //"GUScast-e3dba6d1a50685debc3f78702890a67d".to_owned(),
            "tm-000001111114".to_owned(),
            8443,
            //&["path=/"],
            //&[r#"rs=" "nf=1" "bs=FA8FCA951A6C", "st=0" "ca=199172" "fn=Nest sala" "ic=/setup/icon.png" "md=Google Nest Mini" "ve=05" "rm=27F536D61928EBBD" "cd=9D299FBF152E8D7BDFF0948C3AB696BB" "id=b68fd34f4af1c281aca83a84f0105393"#],
            &[r#"txtvers=1" "dcap=/dcap" "path=/tm" "https=8443" "level=-S1"#],
        );
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
 */
 pub fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("libmdns=debug");
    builder.init();

    let responder = libmdns::Responder::new().unwrap();
    
    loop {
        let _svc = responder.register(
            //"_googlecast._tcp".to_owned(),
            //"_smartenergy._tcp".to_owned(),
            "tm._sub._smartenergy._tcp".to_owned(),
            //"GUScast-e3dba6d1a50685debc3f78702890a67d".to_owned(),
            "tm-000001111114".to_owned(),
            8443,
            //&["path=/"],
            //&[r#"rs=" "nf=1" "bs=FA8FCA951A6C", "st=0" "ca=199172" "fn=Nest sala" "ic=/setup/icon.png" "md=Google Nest Mini" "ve=05" "rm=27F536D61928EBBD" "cd=9D299FBF152E8D7BDFF0948C3AB696BB" "id=b68fd34f4af1c281aca83a84f0105393"#],
            &[r#"txtvers=1" "dcap=/dcap" "path=/tm" "https=8443" "level=-S1"#],
        );
        /*
        let _svc2 = responder.register(
            //"_googlecast._tcp".to_owned(),
            "_smartenergy._tcp".to_owned(),
            //"GUScast-e3dba6d1a50685debc3f78702890a67d".to_owned(),
            "edev-000001111114".to_owned(),
            8443,
            //&["path=/"],
            //&[r#"rs=" "nf=1" "bs=FA8FCA951A6C", "st=0" "ca=199172" "fn=Nest sala" "ic=/setup/icon.png" "md=Google Nest Mini" "ve=05" "rm=27F536D61928EBBD" "cd=9D299FBF152E8D7BDFF0948C3AB696BB" "id=b68fd34f4af1c281aca83a84f0105393"#],
            &[r#"txtvers=1" "dcap=/dcap" "path=/edev" "https=8443" "level=-S1"#],
        );
         */
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}