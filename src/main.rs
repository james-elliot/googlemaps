/*
If the number is greater than 1800000000 (for latitude, also comparing to 900000000 would work) you need to subtract 2^32 (=4294967296) and you get the correct latitudeE7 or longitudeE7.
latitudeE7 = 4233738877 - 4294967296 = -61228419 (= 6.12 South)
longitudeE7 = 1066510714 (= 106.7 East, no conversion here)
*/

//use serde_json::{Result, Value};
use serde_json::Value;
use std::fs;

fn extract<'a>(v:&'a Value,i:usize) -> Option<(f64,f64,&'a str)> {
    let t: &Value = &v["locations"][i];
    let time = match t["timestamp"].as_str() {
	Some(x) => x,
	None => return None
    };
    let mut lat = match t["latitudeE7"].as_i64() {
	Some(x) => x,
	None => return None
    };
    let mut lon = match t["longitudeE7"].as_i64() {
	Some(x) => x,
	None => return None
    };
    if lat > 1800000000 {lat = lat - 4294967296;}
    if lon > 1800000000 {lon = lon - 4294967296;}
    Some(((lat as f64)/10000000.0,(lon as f64)/10000000.0,time))
}

fn open_file(name: &str) -> Value {
    let contents = fs::read_to_string(name).expect("Something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents).expect("Not a valid json content?");
    return v;
}

fn main() {
    let prelude = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.0" creator="JM Alliot" xmlns="http://www.topografix.com/GPX/1/0">
<trk>
    <name>Example GPX Document</name>
    <trkseg>"#;
    let v = open_file("Records.json");
    println!("{}",prelude);
    let mut i = 0;
    loop {
	let (lat,lon,time)=match extract(&v,i) {
	    Some((lat,lon,time)) => (lat,lon,time),
	    None => {println!("</trkseg>\n</trk>\n</gpx>");return ();},
	};
	i=i+1;
	if lat<0. {
	    println!("\t<trkpt lat=\"{}\" lon=\"{}\">",lat,lon);
	    println!("\t\t<time>{}</time>\n\t</trkpt>",time);
	}
    };
}
