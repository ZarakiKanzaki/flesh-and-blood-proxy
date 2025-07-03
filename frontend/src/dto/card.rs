use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Card {
    pub unique_id: String,
    pub name: Option<String>,
    pub pitch: Option<String>,
    pub cost: Option<String>,
    pub power: Option<String>,
    pub defense: Option<String>,
    pub health: Option<String>,
    pub intelligence: Option<String>,
    pub arcane: Option<String>,
    pub types: Vec<String>,

    pub card_keywords: Option<Vec<String>>,

    pub abilities_and_effects: Option<Vec<String>>,

    pub ability_and_effect_keywords: Option<Vec<String>>,

    pub granted_keywords: Option<Vec<String>>,

    pub removed_keywords: Vec<String>,

    pub interacts_with_keywords: Option<Vec<String>>,

    pub functional_text: Option<String>,

    pub functional_text_plain: Option<String>,

    pub type_text: Option<String>,

    pub played_horizontally: Option<bool>,

    pub blitz_legal: Option<bool>,

    pub cc_legal: Option<bool>,

    pub commoner_legal: Option<bool>,

    pub ll_legal: Option<bool>,

    pub blitz_living_legend: bool,

    pub cc_living_legend: Option<bool>,

    pub blitz_banned: Option<bool>,

    pub cc_banned: Option<bool>,

    pub commoner_banned: Option<bool>,

    pub ll_banned: Option<bool>,

    pub upf_banned: Option<bool>,

    pub blitz_suspended: Option<bool>,

    pub cc_suspended: Option<bool>,

    pub commoner_suspended: Option<bool>,

    pub ll_restricted: Option<bool>,

    pub printings: Option<Vec<Printing>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Printing {
    pub unique_id: String,
    pub set_printing_unique_id: Option<String>,
    pub id: Option<String>,
    pub set_id: Option<String>,
    pub edition: Option<String>,
    pub foiling: Option<String>,
    pub rarity: Option<String>,
    pub expansion_slot: Option<bool>,
    pub artists: Option<Vec<String>>,
    pub art_variations: Option<Vec<String>>,
    pub flavor_text: Option<String>,
    pub flavor_text_plain: Option<String>,
    pub image_url: Option<String>,
    pub image_rotation_degrees: Option<i32>,
    pub tcgplayer_product_id: Option<String>,
    pub tcgplayer_url: Option<String>,
}