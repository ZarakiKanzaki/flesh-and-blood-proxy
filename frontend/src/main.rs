mod app;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    color_eyre::install()?;
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
    Ok(())
}