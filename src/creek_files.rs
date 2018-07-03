use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum CreekFileType {
    EgkAllgemein,
    EgkGeschuetzt,
    EgkPersoenlich,
    EgkMFDFHCAEF,
    EgkPruefungsnachweis,
    EgkMFEFGDO,
    EgkResult,
    KvkDaten
}
lazy_static! {
    pub static ref FILENAMES: HashMap<CreekFileType, &'static str> = {
        let mut map = HashMap::new();
        map.insert(CreekFileType::EgkAllgemein, "eGK_allgemeineVersicherungsdaten.xml");
        map.insert(CreekFileType::EgkGeschuetzt, "eGK_geschuetzteVersichertendaten.xml");
        map.insert(CreekFileType::EgkPersoenlich, "eGK_PersoenlicheVersichertendaten.xml");
        map.insert(CreekFileType::EgkMFDFHCAEF, "eGK_MFDF_HCA_EF_StatusVD.xml");
        map.insert(CreekFileType::EgkPruefungsnachweis, "eGK_Pruefungsnachweis.xml");
        map.insert(CreekFileType::EgkMFEFGDO, "eGK_MFEFGDO.xml");
        map.insert(CreekFileType::EgkResult, "Result.xml");
        map.insert(CreekFileType::KvkDaten, "KVK_Daten.bin");
        map
    };
}
