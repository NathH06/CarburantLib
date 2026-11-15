//! Tests unitaires de CarburantLib.
//!
//! À activer dans `src/lib.rs` :
//!
//! ```rust
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! Les tests dépendant des données utilisent un fixture JSON temporaire
//! écrit dans `data_carburant.json`, puis supprimé automatiquement à la fin
//! du test. Aucun `#[ignore]` n'est donc nécessaire pour eux : la CI
//! GitHub Actions les exécute. Seul le test réseau reste ignoré
//! (`cargo test -- --ignored` pour le lancer).

#[cfg(test)]
use crate::*;

#[cfg(test)]
use std::sync::{Mutex, MutexGuard, OnceLock};

/// Nom du fichier que lit `get_json_data()` (cf. src/data.rs).
#[cfg(test)]
const DATA_FILE: &str = "data_carburant.json";

#[cfg(test)]
struct FileCleanup(&'static str);

#[cfg(test)]
impl Drop for FileCleanup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.0);
    }
}

// ---------------------------------------------------------------------------
// Fixture : jeu de données minimal et déterministe
// ---------------------------------------------------------------------------

/// Les tests Rust s'exécutent en parallèle dans des threads. Comme tous
/// écrivent/lisent le même `data_carburant.json`, ce Mutex garantit qu'un
/// seul test l'utilise à la fois.
#[cfg(test)]
fn data_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// Écrit un fichier `data_carburant.json` de test correspondant aux
/// assertions ci-dessous, et retourne un garde qui :
///   1. verrouille l'accès (tests parallèles), et
///   2. supprime le fichier à la fin du test.
///
/// /!\ Le fichier réel `data_carburant.json` (s'il existe) est écrasé
/// le temps du test, puis supprimé.
#[cfg(test)]
fn setup_test_data() -> (MutexGuard<'static, ()>, FileCleanup) {
    let guard = data_lock();

    let fixture = r#"[
        {
            "ville": "Gignac",
            "adresse": "Route de Pézenas",
            "cp": "34150",
            "latitude": "43.657",
            "longitude": "3.605",
            "carburants_disponibles": ["Gazole", "SP95"],
            "gazole_prix": 1.899,
            "sp95_prix": 1.949
        },
        {
            "ville": "Fréjus",
            "adresse": "AVENUE DE PROVENCE",
            "cp": "83600",
            "latitude": "43.433",
            "longitude": "6.737",
            "carburants_disponibles": ["Gazole", "SP98"],
            "gazole_prix": 2.033,
            "sp98_prix": 2.199
        },
        {
            "ville": "Fréjus",
            "adresse": "ROUTE DE BARGEMON",
            "cp": "83600",
            "latitude": "43.449",
            "longitude": "6.740",
            "carburants_disponibles": ["Gazole"],
            "gazole_prix": 2.087
        }
    ]"#;

    std::fs::write(DATA_FILE, fixture).expect("écriture du fixture de test");
    (guard, FileCleanup(DATA_FILE))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_metadata_and_constants() {
    assert_eq!(get_data_author(), "Ministères économiques et financiers.");
    assert_eq!(get_lib_author(), "Nathan H");

    let carburants = get_carburant_list();
    assert_eq!(carburants.len(), 6);
    assert!(carburants.contains(&"Gazole"));
    assert!(carburants.contains(&"E85"));
}

#[test]
fn test_json_loading_and_cities() {
    let _guard = setup_test_data();

    let data = get_json_data().expect("lecture du JSON OK");
    assert!(data.is_array(), "Le JSON racine doit être un tableau");

    let cities = get_city_list().expect("liste des villes OK");
    assert!(!cities.is_empty(), "La liste des villes ne doit pas être vide");
    // Vérifie que la ville "Gignac" présente dans le fixture est bien trouvée
    assert!(cities.iter().any(|c| c.as_str() == Some("Gignac")));
}

#[test]
fn test_search_functions() {
    let _guard = setup_test_data();

    // Test recherche par ville
    let stations_frejus = search_by_city("Fréjus").expect("recherche OK");
    assert_eq!(stations_frejus.len(), 2, "Devrait trouver les 2 stations de Fréjus");

    // Test recherche par code postal
    let stations_cp = search_by_postal_code("34150").expect("recherche OK");
    assert_eq!(stations_cp.len(), 1);
    assert_eq!(stations_cp[0]["ville"], "Gignac");

    // Test recherche par adresse
    let stations_addr = search_by_adresse("Route de Pézenas").expect("recherche OK");
    assert_eq!(stations_addr.len(), 1);
}

#[test]
fn test_get_lower_price() {
    let _guard = setup_test_data();

    // 1. Prix le plus bas du Gazole parmi les stations de Fréjus
    let stations_frejus = search_by_city("Fréjus").expect("recherche OK");
    let best_gazole = get_lower_price(&stations_frejus, "Gazole");

    assert!(best_gazole.disponibility);
    assert_eq!(best_gazole.ville, "Fréjus");
    assert_eq!(best_gazole.adresse, "AVENUE DE PROVENCE");
    assert!((best_gazole.prix - 2.033).abs() < 1e-6);

    // 2. Carburant disponible uniquement à une station (SP98)
    let best_sp98 = get_lower_price(&stations_frejus, "SP98");
    assert!(best_sp98.disponibility);
    assert!((best_sp98.prix - 2.199).abs() < 1e-6);

    // 3. Carburant absent de la zone (SP95 n'est vendu qu'à Gignac)
    let best_sp95 = get_lower_price(&stations_frejus, "SP95");
    assert!(!best_sp95.disponibility);
    assert_eq!(best_sp95.prix, 0.0);

    // 4. Carburant totalement inexistant
    let best_inconnu = get_lower_price(&stations_frejus, "Diesel");
    assert!(!best_inconnu.disponibility);
    assert_eq!(best_inconnu.prix, 0.0);
}

#[test]
#[ignore = "nécessite une connexion réseau et écrase data_carburant.json"]
fn test_update_data_network() {
    let _cleanup = FileCleanup(DATA_FILE);
    update_data().expect("téléchargement OK");
    assert!(std::path::Path::new(DATA_FILE).exists());
}

// Ce fichier est fait entièrement avec une IA afin de gagner du temps sur les tests unitaires. Il est possible que certains tests ne soient pas fiables à 100% dans ce cas indiquez le moi pour que je les corrige.
// Les autres fichiers sont réalisés par Nathan H avec parfois de l'aide d'une IA si nécessaire.