use crate::config::URL_DATA;
use crate::error::DataError;
use std::io::BufReader;
use std::io;
use std::fs::File;
use reqwest::blocking;
use serde_json::Value;

const DATA_FILE: &str = "data_carburant.json";

pub fn update_data() -> Result<(), DataError> {
    let client: blocking::Client = blocking::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let mut resp: blocking::Response = client.get(URL_DATA).send()?.error_for_status()?;
    let mut out: File = File::create(DATA_FILE)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}

pub fn get_json_data() -> Result<Value, DataError> {
    let file: File = File::open(DATA_FILE)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}