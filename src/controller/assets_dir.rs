use tower_http::services::ServeDir;


pub fn init() -> ServeDir {
    let assets_path = std::env::current_dir().unwrap();

    ServeDir::new( format!( "{}/assets" , assets_path.to_str().unwrap() ) )
}
