mod types;
mod game;
mod websocket;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use game::{GameEngine, MapGenerator};
use websocket::{GameSession, websocket_handler};
use types::*;

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        // Entity types
        Territory,
        Player,
        TerrainType,
        BuildingType,
        AIPersonality,
        GameState,
        CombatResult,
        GameStats,
        NotificationLevel,
        // Message types
        ClientMessage,
        ServerMessage,
    )),
    tags(
        (name = "strategy-game", description = "Strategy game API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Generate game
    let map_gen = MapGenerator::new(75, 9); // 75 territories, 9 players (1 human + 8 AI)
    let initial_state = map_gen.generate();

    // Create game engine
    let engine = GameEngine::new(initial_state, 100); // 100ms tick rate

    // Create game session
    let game_session = Arc::new(GameSession::new(engine));

    // Start game loop
    game_session.clone().start_game_loop().await;

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(game_session);

    // Start server
    let addr = "0.0.0.0:3000";
    println!("🎮 Strategy Game Server running on {}", addr);
    println!("📚 Swagger UI: http://localhost:3000/swagger-ui");
    println!("🔌 WebSocket: ws://localhost:3000/ws");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
