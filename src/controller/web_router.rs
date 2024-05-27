use axum::Router;

pub fn init() -> Router<> {
    Router::new()
        .route( "/"        , get( hello   ) )
        .route( "/goodbye" , get( goodbye ) )
}
