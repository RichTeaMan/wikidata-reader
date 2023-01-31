use flate2::bufread::MultiGzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let file_name = "/media/tom/ca202c1a-3e09-4329-b1b1-c056c1c59fd3/latest-all.json.gz";

    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let d = MultiGzDecoder::new(f);

    let mut previous = false;

    println!("[");
    for line in io::BufReader::new(d).lines() {
        if let Ok(l) = line {
            if !l.contains(r#""type":"item""#) {
                // property
                let x: &[_] = &[',', '[', ']', ' '];
                let trimmed = l.trim_matches(x);
                if trimmed.len() == 0 {
                    continue;
                }

                if previous {
                    println!(",");
                }
                print!("{}", trimmed);
                previous = true;
            }
        } else {
            println!("{e:?}", e = line.err().unwrap());
        }
    }
    println!();
    println!("]");

    eprintln!("complete.");
}

fn fetch_entity_id(json_str: &str) -> Option<String> {
    let mut result = None;
    if let Ok(entity) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&json_str) {
        let entity_key_opt = if let Some((_, id)) = entity.get_key_value("id") {
            if let Some(id_str) = id.as_str() {
                Some(id_str.to_owned().clone())
            } else {
                None
            }
        } else {
            None
        };
        result = entity_key_opt.clone();
    }
    result
}
