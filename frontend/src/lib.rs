use yew::prelude::*;


mod components;
mod dto;

use components::card_search::CardSearch;

#[function_component]
pub fn App() -> Html {
    html!(<CardSearch />)
}
