mod card;
mod app;

use struct_to_json_db::set_struct_json_path;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    color_eyre::install()?;

    set_struct_json_path("./db/");

    yew::Renderer::<app::App>::new().render();

    Ok(())
}