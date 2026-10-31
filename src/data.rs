use crate::config::URL_DATA;
use std::io::BufReader;
use std::io;
use std::fs::File;
use reqwest::blocking;
use serde_json::Value;

const DATA_FILE: &str = "data_carburant.json";

pub fn update_data() {
    let client: blocking::Client = blocking::Client::new();

    let mut resp:blocking::Response = client.get(URL_DATA).send().expect("Request is invalid !");
    let mut out: File = File::create(DATA_FILE).expect("File is not created !");
    io::copy(&mut resp, &mut out).expect("failed to copy !");
}

pub fn get_json_data() -> Value {
    let file: File = File::open(DATA_FILE).expect("failed to open file !");
    let reader: BufReader<File> = BufReader::new(file);
    serde_json::from_reader(reader).expect("failed to convert data !")
}