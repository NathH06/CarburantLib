use serde_json::Value;

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
    let carb_option: &str = match carburant_type {
        "Gazole" => "gazole_prix",
        "SP95" => "sp95_prix",
        "SP98" => "sp98_prix",
        "E10" => "e10_prix",
        "E85" => "e85_prix",
        "GPLc" => "gplc_prix",
        _ => return CarburantStruct::default()
    };

    stations.iter().filter_map(|station| {
        let price:f64 = station.get(carb_option)?.as_f64()?;
        let is_dispo = station.get("carburants_disponibles")?.as_array()?.iter().any(|it| it.as_str() == Some(carburant_type));

        if !is_dispo {
            return None;
        }

        Some((price, station))
    }).min_by(|(price_a, _), (price_b, _)| price_a.total_cmp(price_b)).map(|(price, station)| CarburantStruct {
        adresse: station.get("adresse").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        disponibility: true,
        type_name: carburant_type.to_string(),
        prix: price,
        ville: station.get("ville").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        lat: station.get("latitude").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        long: station.get("longitude").and_then(|v| v.as_str()).unwrap_or("").to_string(),
    }).unwrap_or_default()
}