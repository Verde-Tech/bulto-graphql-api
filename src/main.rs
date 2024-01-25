use env_logger;
use log::info;
use actix_web::{HttpServer, App, middleware, web::{self, Data}, guard};
use gateway::{schema, playground_route};

use async_graphql_actix_web::GraphQL;

mod gateway;
mod remote;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema())),   
            )
            .service(web::resource("/playground").guard(guard::Get()).to(playground_route))
    });

    let url = "127.0.0.1:4000";
    println!("HTTP Server is running! Visit: http://{}", url);
    server.bind(url).unwrap().run().await
}
