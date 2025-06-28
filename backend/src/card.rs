
use serde::{Deserialize, Serialize};
use poem_openapi::Object;

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct Card {
    pub unique_id: String,
    pub name: String,
    pub pitch: Option<String>,
    pub cost: Option<String>,
    pub power: Option<String>,
    pub defense: Option<String>,
    pub health: Option<String>,
    pub intelligence: Option<String>,
    pub arcane: Option<String>,
    pub types: Vec<String>,

    #[serde(rename = "card_keywords")]
    pub card_keywords: Option<Vec<String>>,

    #[serde(rename = "abilities_and_effects")]
    pub abilities_and_effects: Option<Vec<String>>,

    #[serde(rename = "ability_and_effect_keywords")]
    pub ability_and_effect_keywords: Option<Vec<String>>,

    #[serde(rename = "granted_keywords")]
    pub granted_keywords: Option<Vec<String>>,

    #[serde(rename = "removed_keywords")]
    pub removed_keywords: Vec<String>,

    #[serde(rename = "interacts_with_keywords")]
    pub interacts_with_keywords: Option<Vec<String>>,

    #[serde(rename = "functional_text")]
    pub functional_text: String,

    #[serde(rename = "functional_text_plain")]
    pub functional_text_plain: String,

    #[serde(rename = "type_text")]
    pub type_text: Option<String>,

    #[serde(rename = "played_horizontally")]
    pub played_horizontally: Option<bool>,

    #[serde(rename = "blitz_legal")]
    pub blitz_legal: Option<bool>,

    #[serde(rename = "cc_legal")]
    pub cc_legal: Option<bool>,

    #[serde(rename = "commoner_legal")]
    pub commoner_legal: Option<bool>,

    #[serde(rename = "ll_legal")]
    pub ll_legal: Option<bool>,

    #[serde(rename = "blitz_living_legend")]
    pub blitz_living_legend: bool,

    #[serde(rename = "cc_living_legend")]
    pub cc_living_legend: Option<bool>,

    #[serde(rename = "blitz_banned")]
    pub blitz_banned: Option<bool>,

    #[serde(rename = "cc_banned")]
    pub cc_banned: Option<bool>,

    #[serde(rename = "commoner_banned")]
    pub commoner_banned: Option<bool>,

    #[serde(rename = "ll_banned")]
    pub ll_banned: Option<bool>,

    #[serde(rename = "upf_banned")]
    pub upf_banned: Option<bool>,

    #[serde(rename = "blitz_suspended")]
    pub blitz_suspended: Option<bool>,

    #[serde(rename = "cc_suspended")]
    pub cc_suspended: Option<bool>,

    #[serde(rename = "commoner_suspended")]
    pub commoner_suspended: Option<bool>,

    #[serde(rename = "ll_restricted")]
    pub ll_restricted: Option<bool>,

    pub printings: Option<Vec<Printing>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Object)]

pub struct Printing {
    pub unique_id: String,
    #[serde(rename = "set_printing_unique_id")]
    pub set_printing_unique_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "set_id")]
    pub set_id: Option<String>,
    pub edition: Option<String>,
    pub foiling: Option<String>,
    pub rarity: Option<String>,
    #[serde(rename = "expansion_slot")]
    pub expansion_slot: Option<bool>,
    pub artists: Option<Vec<String>>,
    #[serde(rename = "art_variations")]
    pub art_variations: Option<Vec<String>>,
    #[serde(rename = "flavor_text")]
    pub flavor_text: Option<String>,
    #[serde(rename = "flavor_text_plain")]
    pub flavor_text_plain: Option<String>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "image_rotation_degrees")]
    pub image_rotation_degrees: Option<i32>,
    #[serde(rename = "tcgplayer_product_id")]
    pub tcgplayer_product_id: Option<String>,
    #[serde(rename = "tcgplayer_url")]
    pub tcgplayer_url: Option<String>,
}