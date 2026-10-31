use crate::{data::get_json_data, error::SearchError};
use serde_json::Value;

fn search_by(search_type: &str, search_data: &str, subdata: bool) -> Result<Vec<Value>, SearchError> {
    let datas: Value = get_json_data()?;
    let mut res:Vec<Value> = vec![];

    if let Some(data_array) = datas.as_array() {
        for data in data_array.iter() {
            let val_str = data.get(search_type).and_then(|v| v.as_str()).unwrap_or("");

            if search_data.is_empty() || val_str == search_data {
                if subdata {
                    res.push(data.clone());
                } 
                else {
                    if let Some(field_val) = data.get(search_type) {
                        res.push(field_val.clone());
                    }
                }
            }
        }
    }
    Ok(res)
}

pub fn get_city_list() -> Result<Vec<Value>, SearchError> {
    let mut res: Vec<Value> = search_by("ville", "", false)?;

    res.sort_by_key(|v| v.to_string());
    res.dedup();
    Ok(res)
}

pub fn search_by_city(city: &str) -> Result<Vec<Value>, SearchError> {
    search_by("ville", city, true)
}

pub fn search_by_adresse(adresse: &str) -> Result<Vec<Value>, SearchError> {
    search_by("adresse", adresse, true)
}

pub fn search_by_postal_code(cp: &str) -> Result<Vec<Value>, SearchError> {
    search_by("cp", cp, true)
}