use crate::{data::get_json_data, error::SearchError};
use serde_json::Value;


fn get_data_by_subdata(data: &Value, data_field_val: Option<&Value>, subdata: bool) -> Option<Value> {
    if subdata {
        Some(data.clone())
    } else {
        data_field_val.cloned()
    }
}

fn search_by(search_type: &str, search_data: &str, subdata: bool) -> Result<Vec<Value>, SearchError> {
    let json_data: Value = get_json_data()?;

    let Some(data_array) = json_data.as_array() else{
        return Err(SearchError::InvalidData("Invalid data format".into()));
    };

    Ok(data_array.iter().filter_map(|data| {
        let data_field_val: Option<&Value> = data.get(search_type);
        let val_str:Option<&str> = data_field_val.and_then(|v| v.as_str());

        if search_data == "*"{
            return get_data_by_subdata(data, data_field_val, subdata);
        }

        if val_str != Some(search_data) {
            return None;
        }

        get_data_by_subdata(data, data_field_val, subdata)
    }).collect())
}

pub fn get_city_list() -> Result<Vec<Value>, SearchError> {
    let mut res: Vec<Value> = search_by("ville", "*", false)?;

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