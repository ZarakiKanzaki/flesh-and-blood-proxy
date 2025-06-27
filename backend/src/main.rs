

struct CardsApi;

#[OpenApi]
impl CardsApi {
    #[oai(path = "/todos", method = "post")]
    async fn create(
        &self,
        pool: Data<&SqlitePool>,
    ) -> Result<Json<i64>> {
        
        Ok(Json("Heloooo"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite:cards.db").await?;
    let api_service = 
	    OpenApiService::new(CardsApi, "Cards", "1.0.0")
	    .server("http://localhost:3000");
    let ui = api_service.openapi_explorer();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .data(pool);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route).await?;
    Ok(())
}
