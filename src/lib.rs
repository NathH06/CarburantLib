pub use serde_json::Value;

mod config;
mod data;
mod price;
mod search;
mod error;

pub use config::{get_carburant_list, get_data_author, get_lib_author};
pub use data::{get_json_data, update_data};
pub use price::{get_lower_price, CarburantStruct};
pub use search::{get_city_list, search_by_adresse, search_by_city, search_by_postal_code};
pub use error::{DataError, SearchError};