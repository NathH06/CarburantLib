pub const URL_DATA: &str = "https://www.data.gouv.fr/api/1/datasets/r/b0561905-7b5e-4f38-be50-df05708acb80";
const DATA_AUTHOR: &str = "Ministères économiques et financiers.";
const LIB_AUTHOR: &str = "Nathan H";

pub fn get_data_author() -> &'static str {
    DATA_AUTHOR
}

pub fn get_lib_author() -> &'static str {
    LIB_AUTHOR
}

pub fn get_carburant_list() -> Vec<&'static str> {
    vec![
        "Gazole",
        "SP95",
        "SP98",
        "E10",
        "E85",
        "GPLc"
    ]
}