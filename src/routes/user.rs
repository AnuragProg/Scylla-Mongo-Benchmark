use serde_json::json;
use crate::models::{UserQuery, UserInsertRequests, UserResponse};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::{content::RawJson, status::Custom};
use crate::repository::UserRepository;

#[get("/?<query..>")]
pub async fn get_users(query: UserQuery, user_repository: &rocket::State<Box<dyn UserRepository>>) -> Custom<RawJson<String>>{
    let response : Result<UserResponse, String> = match query {
        UserQuery{ name: Some(name), age: Some(age), next_page_token} => user_repository.get_users_by_name_age(name, age, next_page_token).await.map_err(|err| err.to_string()),
        UserQuery { name: Some(name), age: None, next_page_token } => user_repository.get_users_by_name(name, next_page_token).await.map_err(|err| err.to_string()),
        UserQuery { name: None, age: Some(age), next_page_token } => user_repository.get_users_by_age(age, next_page_token).await.map_err(|err| err.to_string()),
        _ =>  Err("need at least one of name and age".to_string())
    };
    match response{
        Ok(users) => Custom(Status::Ok, RawJson(json!(users).to_string())),
        Err(msg) => Custom(Status::InternalServerError, RawJson(json!({"status": "error", "message": msg}).to_string())) ,
    }
}

#[post("/", data="<users>", format="json")]
pub async fn save_users(user_repository: &rocket::State<Box<dyn UserRepository>>, users: Json<UserInsertRequests>) -> Custom<RawJson<String>>{
    match user_repository.insert_users(users.into_inner()).await{
        Ok(_) => Custom(Status::Ok, RawJson(json!({"status": "ok"}).to_string())),
        Err(err) => Custom(Status::InternalServerError, RawJson(json!({"message": err.to_string()}).to_string()))
    }
}
