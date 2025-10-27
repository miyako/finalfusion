use axum::{
    extract::{State},
    http::{StatusCode, header},
    response::{Response, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    compression::CompressionLayer,
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter};
use std::fs::File;
use std::io::BufReader;
use finalfusion::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
use clap::Parser;
use ndarray::{Axis,Array2};

#[derive(Serialize)]
struct WordVector<'a> {
    word: &'a str,
    vector: Vec<f32>,
}

#[derive(Serialize)]
struct WordVectors<'a> {
    aggregate: Vec<f32>,
    words: Vec<WordVector<'a>>,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    #[arg(short, long)]
    model: String,
}

#[derive(Clone)]
struct AppState {
    embeddings: Arc<Embeddings<VocabWrap, StorageWrap>>,  
    model: Arc<String>,  
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    
    let cli = Cli::parse();
    let port = cli.port;
    let model_path: &str = &cli.model; 
    let mut reader = BufReader::new(File::open(model_path).unwrap());
    
    let path = std::path::Path::new(&model_path);
    let model_name = path
    .file_name()
    .and_then(|f| f.to_str())
    .unwrap_or("");
    let model: Arc<String> = Arc::new(model_name.to_string());
    /*
    let model_ext = path
    .extension()
    .and_then(|e| e.to_str())
    .unwrap_or("");
    */
    // Set up tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    if !path.exists() {
        error!("File does not exist!");
    } 

    let embeddings = Arc::new(Embeddings::read_embeddings(&mut reader).unwrap());
    
    let state = AppState { embeddings, model };

    // Build router
    let app = Router::new()
        .route("/model", get(get_model))
        .with_state(state.clone())
        .route("/embeddings", post(post_embeddings))
        .with_state(state.clone())
        // Middlewares
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        );

    // Server address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting server on {}", addr);

    // Graceful shutdown signal (Ctrl+C)
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let graceful = async {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("Shutdown signal received (Ctrl+C)");
            }
            _ = shutdown_rx => {
                info!("Shutdown triggered programmatically");
            }
        }
    };

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful);

    if let Err(err) = server.await {
        error!("Server error: {}", err);
    }

    let _ = shutdown_tx.send(());
}

async fn get_model(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let model = state.model;
    let body = serde_json::json!({ "status": "ok", "model": *model });
    (StatusCode::OK, Json(body))
}

async fn post_embeddings(
    State(state): State<AppState>,
    text: String,
) -> impl IntoResponse {

    let embeddings = state.embeddings;    
    let words: Vec<&str> = text.unicode_words().collect();  
    let (emb_matrix, in_vocab_flags) = embeddings.embedding_batch(&words);
    
    if emb_matrix.shape()[0] != words.len() {
        let err = ErrorResponse {
            message: "vector size mismatch".to_string(),
        };
        return (StatusCode::BAD_REQUEST, Json(err)).into_response();
    }else{
        let mut word_vectors: Vec<WordVector> = Vec::with_capacity(words.len());
        for (i, word) in words.iter().enumerate() {
           let in_vocab = in_vocab_flags[i];
           // Extract the i-th row of the Array2<f32>
           let vector: Vec<f32> = if in_vocab {
               emb_matrix
                   .row(i)
                   .to_owned()      // convert ArrayView1<f32> → Array1<f32>
                   .to_vec()         // convert Array1<f32> → Vec<f32>
           } else {
               vec![0.0; embeddings.dims()] // zero vector for OOV words
           };
           word_vectors.push(WordVector {
               word: *word, // deref &&str → &str
               vector,
           });
       }

        let vectors_2d: Vec<Vec<f32>> = word_vectors
       .iter()
       .map(|wv| wv.vector.clone())  // clone each Vec<f32>
       .collect();

        let mut response_data = WordVectors {
            aggregate: Vec::new(),
            words: word_vectors,
       };
       
       if vectors_2d.len() != 0 {
        let rows = vectors_2d.len();
        let cols = vectors_2d[0].len();
        let flattened: Vec<f32> = vectors_2d.into_iter().flatten().collect();
        let result = Array2::from_shape_vec((rows, cols), flattened);
        match result {
            Ok(array) => {
                let mean_axis0 = array.mean_axis(Axis(0)).unwrap(); 
                response_data.aggregate = mean_axis0.to_vec();
            }
            Err(err) => {
                println!("Conversion failed: {}", err);
            }
        }          
       }
         
       let json = match serde_json::to_string(&response_data) {
           Ok(s) => s,
           Err(_) => "{}".to_string(),
       };
       
       return Response::builder()
       .status(StatusCode::OK)
       .header(header::CONTENT_TYPE, "application/json")
       .body(json)
       .unwrap().into_response();
    }
}
