use crate::config::URL_DATA;
use crate::error::DataError;
use std::io::{BufReader, Write, copy};
use std::fs::{File, rename};
use reqwest::blocking;
use serde_json::Value;

const DATA_FILE: &str = "data_carburant.json";
const DATA_FILE_TMP: &str = "data_carburant.tmp";

pub fn update_data() -> Result<(), DataError> {
    let client: blocking::Client = blocking::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let mut resp: blocking::Response = client.get(URL_DATA).send()?.error_for_status()?;

    let mut out: File = File::create(DATA_FILE_TMP)?;
    copy(&mut resp, &mut out)?;
    out.flush()?;
    drop(out);

    rename(DATA_FILE_TMP, DATA_FILE)?;
    Ok(())
}

pub fn get_json_data() -> Result<Value, DataError> {
    let file: File = File::open(DATA_FILE)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}