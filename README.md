# CarburantLib
The library for CarburantVerif application written in Rust and is created by Nathan H.

### Description
This library uses data from the French government to provide information about fuel prices and availability at gas stations across France. It allows developers to easily integrate fuel price data into their applications.

### Features
- Fetch fuel prices from the French government API.
- Retrieve information about gas stations, including location and available fuel types.
- Support for multiple fuel types, including:
    - SP95
    - SP98
    - E10
    - E85
    - Diesel
- get lowest fuel prices for a specific fuel type and location.

### Installation
To use CarburantLib in your Rust project, download the library here (not yet published on crates.io). You can include it in your `Cargo.toml` file as follows:

```toml
[dependencies]
carburant_lib = { path = "path/to/carburant_lib" }
```

### Usage steps
1. Import the library in your Rust code:
```rust
use carburant_lib;
```
2. You can use this functions:
- get_lower_price(stations: Vec<Value>, carburant_type: &str) -> CarburantStruct
- get_carburant_list () -> Vec<&'static str>
- get_data_author() -> &'static str
- get_lib_author() -> &'static str
- update_data() -> void
- get_json_data() -> Value
- get_city_list() -> Vec<Value>
- search_by_city(city: &str) -> Vec<Value>
- search_by_adresse(adresse: &str) -> Vec<Value>
- search_by_postal_code(cp: &str) -> Vec<Value>

### Future features
- coming soon: get_higher_price(stations: Vec<Value>, carburant_type: &str) -> CarburantStruct
- coming soon: get_average_price(stations: Vec<Value>, carburant_type: &str) -> CarburantStruct

### Authors and acknowledgment
- Author of the library: Nathan H
- Author of the data: French government Ministères économiques et financiers (https://www.data.gouv.fr/api/1/datasets/r/b0561905-7b5e-4f38-be50-df05708acb80)
- All french stations for sharing their data with the public.

### warning
This library is created for educational purposes only. The author is not responsible for any misuse of the data or any consequences that may arise from using this library. Please use the data responsibly and in accordance with applicable laws and regulations.