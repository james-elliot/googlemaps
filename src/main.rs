/*
If the number is greater than 1800000000 (for latitude, also comparing to 900000000 would work) you need to subtract 2^32 (=4294967296) and you get the correct latitudeE7 or longitudeE7.
latitudeE7 = 4233738877 - 4294967296 = -61228419 (= 6.12 South)
longitudeE7 = 1066510714 (= 106.7 East, no conversion here)
*/

use serde_json::Value;
use std::fs;
use chrono::prelude::*;
use argparse::{ArgumentParser, Store};

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
    let mut name = "Records.json".to_string();
    let mut ds = "2010-01-01".to_string();
    let mut de = "2100-12-31".to_string();
    
    { // this block limits scope of borrows by ap.refer() method
	let mut ap = ArgumentParser::new();
	ap.set_description("Convert google maps timeline to gpx");
	ap.refer(&mut ds)
	    .add_option(&["-s","--start-date"], Store,
			"Start date in Y-M-D format (default: 2010-01-01)");
	ap.refer(&mut de)
	    .add_option(&["-e","--end-date"], Store,
			"End date in Y-M-D format (default: 2100-12-31)");
	ap.refer(&mut name)
	    .add_option(&["-n","--name"], Store,
			"File to read (default: Records.json)");
	ap.parse_args_or_exit();
    }
//    println!("ds={} de={} name={}",ds,de,name);
    let start = NaiveDate::parse_from_str(&ds,"%F").unwrap().and_hms_opt(0, 0, 0).unwrap();
    let end = NaiveDate::parse_from_str(&de,"%F").unwrap().and_hms_opt(23, 59, 59).unwrap();
  //  println!("tmp2={:?}",&start);
    /*
    let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let end = NaiveDate::from_ymd_opt(2023, 12, 12).unwrap().and_hms_opt(23, 59, 59).unwrap();
    */
    let first = DateTime::<Utc>::from_utc(start,Utc);
    let last = DateTime::<Utc>::from_utc(end,Utc);
//    println!("{} {}",first,last);
  
    let prelude = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.0" creator="JM Alliot" xmlns="http://www.topografix.com/GPX/1/0">
<trk>
    <name>Example GPX Document</name>
 "#;
    let v = open_file(&name);
    println!("{}",prelude);
    let mut i = 0;
    let mut dp =  0;
    loop {
	let (lat,lon,time)=match extract(&v,i) {
	    Some((lat,lon,time)) => (lat,lon,time),
	    None => {
//		println!("i={} loc={}",i,&v["locations"][i]);
		match &v["locations"][i] {
		    Value::Null => {
			println!("</trkseg>\n</trk>\n</gpx>");
			return();
		    }
		    _ => {i=i+1;continue;}
		}
	    }
	};
	i=i+1;
	let dt = time.parse::<DateTime<Utc>>().unwrap();
	let dtn = dt.date_naive();
//	let y = dtn.year();
//	let m = dtn.month();
	let d = dtn.day();
	if dt>first && dt<last  {
	    if d!= dp {
		if dp!=0 {println!("</trkseg>\n");}
		dp = d;
		println!("<trkseg>\n");
	    }
	    println!("\t<trkpt lat=\"{}\" lon=\"{}\">",lat,lon);
	    println!("\t\t<time>{}</time>\n\t</trkpt>",time);
	}
    };
}
