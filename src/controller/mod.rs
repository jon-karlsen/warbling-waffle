pub mod app_state;
mod api_router;
mod assets_dir;
mod web_router;


use tokio::net::TcpListener;
use axum::Router;
use std::sync::Arc;


pub fn init_app( state: Arc<self::app_state::AppState> ) -> Router<Arc<self::app_state::AppState>> {
    let web_router = self::web_router::init();
    let api_router = self::api_router::init();
    let assets_dir = self::assets_dir::init();

    web_router
        .nest( "/api" , api_router )
        .with_state( state )
        .nest_service( "/assets" , assets_dir )
}


pub async fn init_listener() -> TcpListener {
    tokio::net::TcpListener::bind( "0.0.0.0:3000" ).await.unwrap()
}
