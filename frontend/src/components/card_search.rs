use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::use_state;
use yew::Callback;
use yew::prelude::*;

use crate::dto::card::Card;
use crate::components::card_display::EcoCardDisplay;


#[function_component]
pub fn CardSearch() -> Html {
    let results :UseStateHandle<Vec<Card>> = use_state(|| vec![]);

    let on_search = {
        let results = results.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            let results = results.clone();
            if query.len() <= 2 {
                results.set(vec![]);
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