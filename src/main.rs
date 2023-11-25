pub mod clients;
pub mod models;
pub mod routes;
#[macro_use] extern crate rocket;


#[get("/")]
async fn health_check() -> &'static str {
    "I'm ok, don't worry!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let scylla_client = match clients::ScyllaClient::new().await{
        Ok(session) => session,
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };
    rocket::build()
        .configure(rocket::Config{
            port: 3000,
            ..rocket::Config::default()
        })
        .manage(scylla_client)
        .mount("/users", routes![routes::save_users, routes::get_users])
        .mount("/health", routes![health_check])
        .launch()
        .await?;
    Ok(())
}
