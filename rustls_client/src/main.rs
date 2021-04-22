/// This is the simplest possible client using rustls that does something useful:
/// it accepts the default configuration, loads some root certs, and then connects
/// to google.com and issues a basic HTTP request.  The response is printed to stdout.
///
/// It makes use of rustls::Stream to treat the underlying TLS connection as a basic
/// bi-directional stream -- the underlying IO is performed transparently.
///
/// Note that `unwrap()` is used to deal with networking errors; this is not something
/// that is sensible outside of example code.

/*
use std::sync::Arc;

use std::io::{stdout, Read, Write};
use std::net::TcpStream;

use rustls;
use webpki;
use webpki_roots;

use rustls::{Connection, RootCertStore};

fn main() {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let config = rustls::ClientConfig::new(root_store, &[], rustls::DEFAULT_CIPHERSUITES);

    let dns_name = webpki::DNSNameRef::try_from_ascii_str("google.com").unwrap();
    let mut conn = rustls::ClientConnection::new(&Arc::new(config), dns_name).unwrap();
    let mut sock = TcpStream::connect("google.com:443").unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    tls.write(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "Current ciphersuite: {:?}",
        ciphersuite.suite
    )
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();
}
*/

//extern crate rustls; // 0.19.0
use rustls;

use io::Read;
use io::Write;
use std::io;

fn main() {
    let mut socket = std::net::TcpStream::connect("www.google.com.br:443").unwrap();
    let mut config = rustls::ClientConfig::new();
    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let arc = std::sync::Arc::new(config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("www.google.com.br").unwrap();
    let mut client = rustls::ClientSession::new(&arc, dns_name);
    let mut stream = rustls::Stream::new(&mut client, &mut socket); // Create stream
                                                                    // Instead of writing to the client, you write to the stream
    //stream
    //    .write(b"GET / HTTP/1.1\r\nConnection: close\r\n\r\n")
    //    .unwrap();
    stream.write(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: www.google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    let mut plaintext = Vec::new();
    stream.read_to_end(&mut plaintext).unwrap();
    io::stdout().write_all(&plaintext).unwrap();
}


