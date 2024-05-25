use axum::{ routing::get , Router };
use tracing::info;
use tracing_subscriber::{ layer::SubscriberExt , util::SubscriberInitExt };


mod view;


use crate::view::hello;


#[ tokio::main ]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with( tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(| _ | "warbling_waffle=debug".into() ) )
        .with( tracing_subscriber::fmt::layer() )
        .init();

    info!( "Initialising app..." );

    let app      = Router::new()
                    .route( "/" , get( hello ));
    let listener = tokio::net::TcpListener::bind( "0.0.0.0:3000" ).await.unwrap();

    info!( "Initialisation complete" );

    axum::serve( listener , app ).await.unwrap();

    info!( "Application shutting down" );

    Ok( () )
}
