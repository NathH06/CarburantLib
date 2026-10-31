use serde_json::Value;

struct PriceListStruct {
    index: usize,
    price: f64
}

#[derive(Default)]
pub struct CarburantStruct {
    pub ville: String,
    pub lat: String,
    pub long: String,
    pub adresse: String,
    pub type_name: String,
    pub disponibility: bool,
    pub prix: f64
}

pub fn get_lower_price(stations: &[Value], carburant_type: &str) -> CarburantStruct {
    let mut carb_price_list: Vec<PriceListStruct> = vec![];
    let mut result_station: CarburantStruct = CarburantStruct::default();
    let carb_option: &str = match carburant_type {
        "Gazole" => "gazole_prix",
        "SP95" => "sp95_prix",
        "SP98" => "sp98_prix",
        "E10" => "e10_prix",
        "E85" => "e85_prix",
        "GPLc" => "gplc_prix",
        _ => return CarburantStruct::default()
    };

    for (i, station) in stations.iter().enumerate() {
        carb_price_list.push(PriceListStruct {index: i, price: station.get(carb_option).and_then(|v| v.as_f64()).unwrap_or(f64::MAX)});
    }

    carb_price_list.sort_by(|a, b| a.price.total_cmp(&b.price));
    
    for price in carb_price_list.iter() {
        if price.price == f64::MAX {
            break;
        }
        
        let is_dispo: bool = stations[price.index].get("carburants_disponibles").and_then(|v| v.as_array()).is_some_and(|list| list.iter().any(|it| it.as_str() == Some(carburant_type)));
        if is_dispo {
            result_station.adresse = stations[price.index].get("adresse").and_then(|v| v.as_str()).unwrap_or("").to_string();
            result_station.disponibility = true;
            result_station.type_name = carburant_type.to_string();
            result_station.prix = price.price;
            result_station.ville = stations[price.index].get("ville").and_then(|v| v.as_str()).unwrap_or("").to_string();
            result_station.lat = stations[price.index].get("latitude").and_then(|v| v.as_str()).unwrap_or("").to_string();
            result_station.long = stations[price.index].get("longitude").and_then(|v| v.as_str()).unwrap_or("").to_string();

            break;
        }
    }

    result_station
}