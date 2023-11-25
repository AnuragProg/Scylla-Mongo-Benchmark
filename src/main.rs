pub mod clients;
pub mod models;
pub mod routes;
pub mod repository;
use clap::Parser;
#[macro_use] extern crate rocket;


#[get("/")]
async fn health_check() -> &'static str {
    "I'm ok, don't worry!"
}

#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long)]
    db: String
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let args = Args::parse();
    let user_repository : Box<dyn repository::UserRepository> = match args.db.as_ref(){
        "scylla" => Box::new(clients::ScyllaClient::new().await.unwrap()),
        "mongo" => Box::new(clients::MongoClient::new().await.unwrap()),
        _ => panic!("not a valid option")
    };
    rocket::build()
        .configure(rocket::Config{
            port: 3000,
            ..rocket::Config::default()
        })
        .manage(user_repository)
        .mount("/users", routes![routes::save_users, routes::get_users])
        .mount("/health", routes![health_check])
        .launch()
        .await?;
    Ok(())
}
