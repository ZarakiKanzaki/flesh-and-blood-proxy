use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::use_state;
use yew::Callback;
use yew::prelude::*;
use regex::Regex;
use yew_markdown::Markdown;

mod dto;

use crate::dto::card::card::Card;

#[derive(PartialEq, Properties)]
pub struct CardProperties {
    cards: Vec<Card>
}

#[derive(Debug)]
enum CardSymbol {
    Resource,
    Chi,
    Power,
    Defense,
    Intelligence,
    Health,
    Tap,
    Untap
} 

impl CardSymbol {
    fn from_letter(letter: &str) -> Option<Self> {
        match letter {
            "r" => Some(CardSymbol::Resource),
            "c" => Some(CardSymbol::Chi),
            "p" => Some(CardSymbol::Power),
            "d" => Some(CardSymbol::Defense),
            "i" => Some(CardSymbol::Intelligence),
            "h" => Some(CardSymbol::Health),
            "t" => Some(CardSymbol::Tap),
            "u" => Some(CardSymbol::Untap),
            _ => None,
        }
    }

    fn as_html(&self) -> String {
        match self {
            CardSymbol::Resource => String::from("<span class='fab-icon resource'></span>"),
            CardSymbol::Chi => String::from("<span class='fab-icon chi'></span>"),
            CardSymbol::Power => String::from("<span class='fab-icon power'></span>"),
            CardSymbol::Defense => String::from("<span class='fab-icon defense'></span>"),
            CardSymbol::Intelligence => String::from("<span class='fab-icon intelligence'></span>"),
            CardSymbol::Health => String::from("<span class='fab-icon health'></span>"),
            CardSymbol::Tap => String::from("<span class='fab-icon tap'></span>"),
            CardSymbol::Untap => String::from("<span class='fab-icon untap'></span>"),
        }
    }

    fn format_text_symbol(text :&str) -> String {
        let re = Regex::new(r"\{([a-zA-Z])\}").unwrap();
        re.replace_all(&text, |caps: &regex::Captures| {
            let letter = &caps[1];
            CardSymbol::from_letter(letter)
                .map(|symbol| symbol.as_html())
                .unwrap_or_else(|| String::new())
        }).to_string()
    }

    
}


enum Pitch {
    Red = 1,
    Yellow = 2,
    Blue = 3,
    None = 4,
}

impl Pitch {
    fn from_str(value: &str) -> Pitch {
        match value {
            "1" => Pitch::Red,
            "2" => Pitch::Yellow,
            "3" => Pitch::Blue,
            &_ => Pitch::None,
        }
    }
}

#[function_component]
pub fn EcoCardDisplay(properties:&CardProperties) -> Html {
    let cards = properties.cards.clone();

    html!{
        <div class="w-col-12 grid grid-cols-3 gap-2">
            { 
                cards.iter().map(|card| {

                let top_border = match card.pitch.clone() {
                    Some(pitch) => match Pitch::from_str(&pitch) {
                            Pitch::Red => String::from("border-top: 4px solid red"),
                            Pitch::Yellow => String::from("border-top: 4px solid yellow"),
                            Pitch::Blue => String::from("border-top: 4px solid blue"),
                            Pitch::None => String::from("border-top: 4px solid white"),
                        },
                    None => String::from("border-top: 4px solid white"),
                };

                let pitch_icon = match card.pitch.clone() {
                    Some(pitch) => match Pitch::from_str(&pitch) {
                            Pitch::Red => html!{ <span><img class="w-6 h-6" src="images/one_pitch.svg" /></span> },
                            Pitch::Yellow => html!{ <span><img class="w-6 h-6" src="images/two_pitch.svg" /></span> },
                            Pitch::Blue => html!{ <span><img class="w-6 h-6" src="images/three_pitch.svg" /></span> },
                            Pitch::None => html!{<div></div>},
                        },
                    None => html!{<div></div>},
                };



                let expected_formatted_text = match &card.functional_text {
                    Some(text) => CardSymbol::format_text_symbol(&text),
                    None => String::from("").into() ,
                };

                html!{
                    <div class="grid grid-cols-5 gap-6 m-8" style={top_border}>
                        <div>
                            { pitch_icon }
                        </div>
                        <div class="col-span-3 text-center">
                            { card.name.clone() } 
                        </div>
                        <div>
                            if card.cost.as_ref().map_or(false, |s| !s.is_empty()){
                                <div class="fab-stat">
                                    <img src="images/resource.svg" class="card-stats-img mr-3x" />
                                    <span> { card.cost.clone() } </span>
                                </div>
                            }
                        </div>
                        <div class="col-span-5">
                            <Markdown src={ expected_formatted_text }/>  
                        </div>
                        <div>
                            if card.power.as_ref().map_or(false, |s| !s.is_empty()) {
                                <div class="fab-stat">
                                    <img src="images/power.svg" class="card-stats-img mr-3x" />
                                    <span> { card.power.clone() } </span>
                                </div>
                            }
                            if card.intelligence.as_ref().map_or(false, |s| !s.is_empty()) {
                                <div class="fab-stat">
                                    <img src="images/intelligence.svg" class="card-stats-img mr-3x" />
                                    <span> { card.intelligence.clone() } </span>
                                </div>
                            }
                        </div>

                        <div class="col-span-3 text-center">
                            { card.type_text.clone() }
                        </div>

                        <div>
                           if card.defense.as_ref().map_or(false, |s| !s.is_empty()){
                                <div class="fab-stat">
                                    <img src="images/defense.svg" class="card-stats-img mr-3x" />
                                    <span> { card.defense.clone() } </span>
                                </div>
                            }

                            if card.health.as_ref().map_or(false, |s| !s.is_empty()) {
                                <div class="fab-stat">
                                    <img src="images/health.svg" class="card-stats-img mr-3x" />
                                    <span> { card.health.clone() } </span>
                                </div>
                            }
                        </div>
                    </div>
  
                }
                
                }).collect::<Html>()
            }
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    let results :UseStateHandle<Vec<Card>> = use_state(|| vec![]);

    let on_search = {
        let results = results.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            let results = results.clone();
            if query.len() <= 2 {
                return ();
            }
            spawn_local(async move {
                let endpoint = String::from("http://localhost:3000");
                let url = format!("{}/search?query={}", endpoint, query);
                let client = reqwest::Client::new();
                let resp = client
                    .get(&url)
                    .header("Access-Control-Allow-Origin", "{endpoint}")
                    .send()
                    .await;
                let cards = match resp {
                    Ok(r) => {
                        let raw_text= r.text().await.unwrap_or_else(|_| "Failed to read response".to_string());
                        let res :Vec<Card>= serde_json::from_str(&raw_text).unwrap();
                        res
                    },
                    Err(_) => vec![],
                };
                results.set(cards);
            });
        })
    };

    html! {
          <div class="w-10/12 h-full border-black border-2 rounded-md hover:shadow-[8px_8px_0px_rgba(0,0,0,1)] bg-white">
                <div class="block cursor-pointer">
                    <article class="w-full h-full justify-center">
                        <div class="px-6 py-5 text-left h-full justify-center">
                            <div class="flex flex-col md:flex-row flex-wrap sm:flex-nowrap items-center sm:space-x-4  p-4">
                                <input type="text" class="input-search" oninput={on_search.clone()} />
                            </div>
                        </div>
                        <EcoCardDisplay cards={(*results).clone()}/>

                    </article>
                </div>
            </div>

        
    }
}
