use serde::{Deserialize, Serialize};
use struct_to_json_db::auto_json_db;

#[auto_json_db]
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub unique_id: String,
    pub name: String,
    pub pitch: String,
    pub cost: String,
    pub power: String,
    pub defense: String,
    pub health: String,
    pub intelligence: String,
    pub arcane: String,
    pub types: Vec<String>,

    #[serde(rename = "card_keywords")]
    pub card_keywords: Vec<String>,

    #[serde(rename = "abilities_and_effects")]
    pub abilities_and_effects: Vec<String>,

    #[serde(rename = "ability_and_effect_keywords")]
    pub ability_and_effect_keywords: Vec<String>,

    #[serde(rename = "granted_keywords")]
    pub granted_keywords: Vec<String>,

    #[serde(rename = "removed_keywords")]
    pub removed_keywords: Vec<String>,

    #[serde(rename = "interacts_with_keywords")]
    pub interacts_with_keywords: Vec<String>,

    #[serde(rename = "functional_text")]
    pub functional_text: String,

    #[serde(rename = "functional_text_plain")]
    pub functional_text_plain: String,

    #[serde(rename = "type_text")]
    pub type_text: String,

    #[serde(rename = "played_horizontally")]
    pub played_horizontally: bool,

    #[serde(rename = "blitz_legal")]
    pub blitz_legal: bool,

    #[serde(rename = "cc_legal")]
    pub cc_legal: bool,

    #[serde(rename = "commoner_legal")]
    pub commoner_legal: bool,

    #[serde(rename = "ll_legal")]
    pub ll_legal: bool,

    #[serde(rename = "blitz_living_legend")]
    pub blitz_living_legend: bool,

    #[serde(rename = "cc_living_legend")]
    pub cc_living_legend: bool,

    #[serde(rename = "blitz_banned")]
    pub blitz_banned: bool,

    #[serde(rename = "cc_banned")]
    pub cc_banned: bool,

    #[serde(rename = "commoner_banned")]
    pub commoner_banned: bool,

    #[serde(rename = "ll_banned")]
    pub ll_banned: bool,

    #[serde(rename = "upf_banned")]
    pub upf_banned: bool,

    #[serde(rename = "blitz_suspended")]
    pub blitz_suspended: bool,

    #[serde(rename = "cc_suspended")]
    pub cc_suspended: bool,

    #[serde(rename = "commoner_suspended")]
    pub commoner_suspended: bool,

    #[serde(rename = "ll_restricted")]
    pub ll_restricted: bool,

    pub printings: Vec<Printing>,
}


#[auto_json_db]
#[derive(Debug, Serialize, Deserialize)]
pub struct Printing {
    pub unique_id: String,
    #[serde(rename = "set_printing_unique_id")]
    pub set_printing_unique_id: String,
    pub id: String,
    #[serde(rename = "set_id")]
    pub set_id: String,
    pub edition: String,
    pub foiling: String,
    pub rarity: String,
    #[serde(rename = "expansion_slot")]
    pub expansion_slot: bool,
    pub artists: Vec<String>,
    #[serde(rename = "art_variations")]
    pub art_variations: Vec<String>,
    #[serde(rename = "flavor_text")]
    pub flavor_text: String,
    #[serde(rename = "flavor_text_plain")]
    pub flavor_text_plain: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "image_rotation_degrees")]
    pub image_rotation_degrees: i32,
    #[serde(rename = "tcgplayer_product_id")]
    pub tcgplayer_product_id: String,
    #[serde(rename = "tcgplayer_url")]
    pub tcgplayer_url: String,
}