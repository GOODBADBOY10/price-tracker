use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct PriceData {
    chain_type: String,
    price_usd: f64,
    price_native: f64,
    market_cap: f64,
    fdv: f64,
    last_updated: String,
}

// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

// App state (we'll use this if we want to add more later)
#[derive(Clone)]
struct AppState {
    prices_file_path: String,
}

#[tokio::main]
async fn main() {
    println!("🚀 API Server starting...");
    
    // Configure the path to prices.json
    let state = AppState {
        prices_file_path: "../worker/prices.json".to_string(),
    };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/prices", get(get_prices))
        .with_state(Arc::new(state));

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("✅ Server running on http://localhost:3000");
    println!("📊 Health check: http://localhost:3000/");
    println!("💰 Prices endpoint: http://localhost:3000/prices");
    println!();

    // Start serving
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "API is running",
        "endpoints": {
            "health": "/",
            "prices": "/prices"
        }
    }))
}

// Get prices endpoint
async fn get_prices(
    State(state): State<Arc<AppState>>,
) -> Result<Json<PriceData>, AppError> {
    // Read the file
    let content = fs::read_to_string(&state.prices_file_path)
        .map_err(|e| AppError::FileNotFound(e.to_string()))?;

    // Parse JSON
    let price_data: PriceData = serde_json::from_str(&content)
        .map_err(|e| AppError::ParseError(e.to_string()))?;

    // Return as JSON
    Ok(Json(price_data))
}

// Custom error type
enum AppError {
    FileNotFound(String),
    ParseError(String),
}

// Implement IntoResponse for our error type
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, user_message) = match self {
            AppError::FileNotFound(msg) => (
                StatusCode::NOT_FOUND,
                "FILE_NOT_FOUND",
                format!("Price data not available yet. Worker might not have run. Details: {}", msg),
            ),
            AppError::ParseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "PARSE_ERROR",
                format!("Failed to parse price data. Details: {}", msg),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
            message: user_message,
        });

        (status, body).into_response()
    }
}