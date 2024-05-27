use askama::Template;
use axum::{ http::StatusCode , response::{ Html , IntoResponse , Response } };


struct HtmlTemplate<T>( T );


impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template {
    fn into_response( self ) -> Response {
        match self.0.render() {
             Ok( h ) => Html( h ).into_response()
          , Err( e ) => (
                StatusCode::INTERNAL_SERVER_ERROR
              , format!( "Failed to render template. Error: {}" , e )
            ).into_response()
        }
    }
}


#[ derive( Template )]
#[ template( path = "hello.html" )]
struct HelloTemplate;


#[ derive( Template )]
#[ template( path = "goodbye.html" )]
struct GoodbyeTemplate;


pub async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate( template )
}


pub async fn goodbye() -> impl IntoResponse {
    let template = GoodbyeTemplate {};
    HtmlTemplate( template )
}
