use flesh_and_blood_proxy_fe::App;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    color_eyre::install()?;
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}