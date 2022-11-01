use serde_json::{Result, Value};
use std::fs;

fn test<'a>(v:&'a Value,i:usize) -> Result<(i64,i64,&'a str)> {
    let t: &Value = &v["locations"][i];
    let time = 	t["timestamp"].as_str().unwrap();
    let lat = 	t["latitudeE7"].as_i64().unwrap();
    let lon = 	t["longitudeE7"].as_i64().unwrap();
    Ok((lat,lon,time))
}

fn main() {
    let contents = fs::read_to_string("Records.json").expect("Something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents).unwrap();
    println!("{}",v["locations"][0]);
//    let t:Value = v["locations"][0].clone();
    let t: &Value = &v["locations"][0];
    let time = 	t["timestamp"].as_str().unwrap();
    let lat = 	t["latitudeE7"].as_i64().unwrap();
    let lon = 	t["longitudeE7"].as_i64().unwrap();
    println!(
	"{} {} {}",
	time,
	lat,
	lon
    );
}
