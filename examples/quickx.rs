extern crate quick_xml;
extern crate reqwest;

use std::str;
use std::io::prelude::*;
use std::io::BufReader;
use quick_xml::reader::Reader;
use quick_xml::events::Event;

fn indent(size: usize) -> String {
    const INDENT: &'static  str = "  ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let url = "http://docsouth.unc.edu/neh/andersrob/andersrob.xml";
    let mut response = BufReader::new(reqwest::get(url).unwrap());
    let mut buf = Vec::with_capacity(8192);
    let mut nsbuf = Vec::with_capacity(64);
    let mut reader = Reader::from_reader(response);
    loop {
        match reader.read_namespaced_event(&mut buf, &mut nsbuf) {
            Ok((ns, Event::Start(ref el))) => {
                ns.map(|n| print!("[{}]", str::from_utf8(n).unwrap()));
                print!(
                    "{} ",
                    str::from_utf8(el.name()).unwrap()
                );
            },
            Ok((_, Event::Eof)) => break,
            Ok(_) => {},
            Err(e) => println!("ERROR! {}", e),
        };
        buf.clear();
    }
}
        

