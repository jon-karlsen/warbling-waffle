mod controller;
mod view;


use tracing::info;
use tracing_subscriber::{ layer::SubscriberExt , util::SubscriberInitExt };
use std::sync::Arc;
use crate::controller::{ app_state , init_app , init_listener };


#[ tokio::main ]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with( tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(| _ | "warbling_waffle=debug".into() ) )
        .with( tracing_subscriber::fmt::layer() )
        .init();

    info!( "Initialising app..." );

    let app_state = Arc::new( app_state::AppState::new( vec![].into() ) );
    let app       = init_app( app_state );
    let listener  = init_listener().await;

    info!( "Initialisation complete" );

    axum::serve( listener , app ).await.unwrap();

    info!( "Application shutting down" );

    Ok( () )
}
