use poem::middleware::Cors;
use poem::EndpointExt;
use poem::{listener::TcpListener,  Route, Server};
use poem_openapi::param::Query;
use poem_openapi::{payload::Json,  OpenApi, OpenApiService};
use tokio::fs;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use tracing::{info};

mod card;

struct Api {
    cards : Vec<card::Card>
}
   
#[OpenApi]
impl Api {
    /// Search Cards
    #[oai(path = "/search", method = "get")]
    async fn seach(&self, #[oai(name = "query")] query: Query<String>)-> Json<Vec<card::Card>> {
        let matcher = SkimMatcherV2::default();
        let mut results: Vec<(card::Card, i64)> = self
                .cards
                .iter()
                .filter_map(|card| {
                    // Perform fuzzy match
                    matcher.fuzzy_match(&card.name, &query).map(|rank| (card.clone(), rank))
                })
                .collect();

         results.sort_by(|(_, rank1), (_, rank2)| rank2.cmp(rank1));

        let sorted_cards: Vec<card::Card> = results.into_iter().map(|(card, _)| card).collect();

        Json(sorted_cards)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    color_eyre::install()?;
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let cards :Vec<card::Card> = serde_json::from_str(&(fs::read_to_string("./db/english/card.json").await?)).expect("JSON was not well-formatted");
    info!("cards found: {:?}", cards.len());
    
    let api_service =
        OpenApiService::new(Api{cards}, "Search cards", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui).with(Cors::new().allow_origin("http://localhost:8080"));

    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
    Ok(())
}