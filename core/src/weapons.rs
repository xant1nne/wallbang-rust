// Config block: weapon key normalisation + display names, 1:1 with Celerity
// `ui/resources/icons/weapon_icons.hpp` (display_table). The icon font is not
// embedded in this build, so the ESP shows the display text.
use std::sync::Arc;

/// `m_szName` from vdata arrives as e.g. `weapon_ak47`. Celerity lower-cases the
/// string and strips the first matching prefix (`weapon_`, `item_`, `c_`).
pub fn normalize_key(raw: &str) -> String {
    let mut key = raw.trim().to_ascii_lowercase();
    for prefix in ["weapon_", "item_", "c_"] {
        if let Some(stripped) = key.strip_prefix(prefix) {
            key = stripped.to_owned();
            break;
        }
    }
    key
}

/// Celerity `weapon_icons::display_table()`.
pub fn display_name(key: &str) -> Option<&'static str> {
    Some(match key {
        "deagle" => "Desert Eagle",
        "elite" => "Dual Berettas",
        "fiveseven" => "Five-SeveN",
        "glock" => "Glock-18",
        "ak47" => "AK-47",
        "aug" => "AUG",
        "awp" => "AWP",
        "famas" => "FAMAS",
        "g3sg1" => "G3SG1",
        "galilar" => "Galil AR",
        "m249" => "M249",
        "m4a1" => "M4A4",
        "mac10" => "MAC-10",
        "p90" => "P90",
        "mp5sd" => "MP5-SD",
        "ump45" => "UMP-45",
        "xm1014" => "XM1014",
        "bizon" => "PP-Bizon",
        "mag7" => "MAG-7",
        "negev" => "Negev",
        "sawedoff" => "Sawed-Off",
        "tec9" => "Tec-9",
        "taser" => "Zeus x27",
        "hkp2000" => "P2000",
        "mp7" => "MP7",
        "mp9" => "MP9",
        "nova" => "Nova",
        "p250" => "P250",
        "shield" => "Shield",
        "scar20" => "SCAR-20",
        "sg556" => "SG 553",
        "ssg08" => "SSG 08",
        "knife" => "Knife",
        "knife_gg" => "Knife",
        "flashbang" => "Flashbang",
        "hegrenade" => "HE Grenade",
        "smokegrenade" => "Smoke Grenade",
        "molotov" => "Molotov",
        "decoy" => "Decoy",
        "incgrenade" => "Incendiary",
        "c4" => "Bomb",
        "knife_t" => "Knife (T)",
        "m4a1_silencer" => "M4A1-S",
        "usp_silencer" => "USP-S",
        "cz75a" => "CZ75-Auto",
        "revolver" => "R8 Revolver",
        "knife_bayonet" => "Bayonet",
        "knife_css" => "Knife",
        "knife_flip" => "Flip Knife",
        "knife_gut" => "Gut Knife",
        "knife_karambit" => "Karambit",
        "knife_m9_bayonet" => "M9 Bayonet",
        "knife_tactical" => "Tactical Knife",
        "knife_falchion" => "Falchion Knife",
        "knife_survival_bowie" => "Bowie Knife",
        "knife_butterfly" => "Butchery Knife",
        "knife_push" => "Shadow Daggers",
        "knife_cord" => "Cord Knife",
        "knife_canis" => "Canis Knife",
        "knife_ursus" => "Ursus Knife",
        "knife_gypsy_jackknife" => "Jack Knife",
        "knife_outdoor" => "Outdoor Knife",
        "knife_stiletto" => "Stiletto Knife",
        "knife_widowmaker" => "Widowmaker Knife",
        "knife_skeleton" => "Skeleton Knife",
        _ => return None,
    })
}

/// Display text for a raw `m_szName`; unknown weapons fall back to the
/// normalised key so the ESP never prints an empty string.
pub fn display_for_raw(raw: &str) -> Option<Arc<str>> {
    if raw.is_empty() {
        return None;
    }
    let key = normalize_key(raw);
    if key.is_empty() {
        return None;
    }
    if let Some(display) = display_name(&key) {
        return Some(Arc::from(display));
    }
    Some(Arc::from(key))
}

#[cfg(test)]
mod tests {
    use super::{display_for_raw, normalize_key};

    #[test]
    fn keys_are_normalised_like_celerity() {
        assert_eq!(normalize_key("weapon_ak47"), "ak47");
        assert_eq!(normalize_key("item_awp"), "awp");
        assert_eq!(normalize_key("c_m4a1_silencer"), "m4a1_silencer");
        assert_eq!(normalize_key("Weapon_AWP"), "awp");
    }

    #[test]
    fn display_names_match_the_table() {
        assert_eq!(display_for_raw("weapon_ak47").as_deref(), Some("AK-47"));
        assert_eq!(display_for_raw("weapon_scar20").as_deref(), Some("SCAR-20"));
        assert_eq!(display_for_raw("weapon_deagle").as_deref(), Some("Desert Eagle"));
        // Unknown weapons keep the normalised key, never an empty string.
        assert_eq!(display_for_raw("weapon_unknown_gun").as_deref(), Some("unknown_gun"));
        assert!(display_for_raw("").is_none());
    }
}
