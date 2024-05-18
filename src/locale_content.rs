pub(crate) const CN: &str = include_str!("../lang/cn.yml");
pub(crate) const DE: &str = include_str!("../lang/de.yml");
pub(crate) const EN: &str = include_str!("../lang/en.yml");
pub(crate) const EO: &str = include_str!("../lang/eo.yml");
pub(crate) const ES: &str = include_str!("../lang/es.yml");
pub(crate) const FR: &str = include_str!("../lang/fr.yml");
pub(crate) const JA: &str = include_str!("../lang/ja.yml");
pub(crate) const KO: &str = include_str!("../lang/ko.yml");
pub(crate) const PL: &str = include_str!("../lang/pl.yml");
pub(crate) const PT: &str = include_str!("../lang/pt.yml");
pub(crate) const SV: &str = include_str!("../lang/sv.yml");
pub(crate) const TR: &str = include_str!("../lang/tr.yml");
pub(crate) const TW: &str = include_str!("../lang/tw.yml");

pub fn get_content(code: String) -> String {
    match code.as_str() {
        "cn" => crate::locale_content::CN,
        "de" => crate::locale_content::DE,
        "en" => crate::locale_content::EN,
        "eo" => crate::locale_content::EO,
        "es" => crate::locale_content::ES,
        "fr" => crate::locale_content::FR,
        "ja" => crate::locale_content::JA,
        "ko" => crate::locale_content::KO,
        "pl" => crate::locale_content::PL,
        "pt" => crate::locale_content::PT,
        "sv" => crate::locale_content::SV,
        "tr" => crate::locale_content::TR,
        "tw" => crate::locale_content::TW,
        _ => crate::locale_content::EN
    }.to_string()
}

pub(crate) const AVAILABLE: &'static [&'static str] = &["cn", "de", "en", "eo", "es", "fr", "ja", "ko", "pl", "pt", "sv", "tr", "tw"];