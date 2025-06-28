use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::use_state;
use yew::Callback;

use yew::prelude::*;


#[function_component]
pub fn App() -> Html {
    let results = use_state(|| String::new());

    let on_search = {
        let results = results.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            let results = results.clone();
            spawn_local(async move {
                let endpoint = String::from("http://localhost:3000");
                let url = format!("{}/search?query={}", endpoint, query);
                let client = reqwest::Client::new();
                let resp = client
                    .get(&url)
                    .header("Access-Control-Allow-Origin", "*")
                    .send()
                    .await;

                let text = match resp {
                    Ok(r) => r.text().await.unwrap_or_else(|_| "Failed to read response".to_string()),
                    Err(_) => "Request failed".to_string(),
                };
                results.set(text);
            });
        })
    };

    html! {
        
                <article class="h-full w-full">
                    <div class="px-6 py-12 text-left h-full">

                        <div class="flex flex-col md:flex-row flex-wrap sm:flex-nowrap items-center sm:space-x-4 w-full p-4">
                            <input type="text" class="input-search" oninput={on_search.clone()} />
                            <p class="w-col-12">{ (*results).as_str() }</p>
                        </div>
                   
                    </div>
                </article>
        
    }
}
