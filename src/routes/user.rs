use serde_json::json;
use scylla::{batch::Batch, QueryResult, };
use crate::clients::ScyllaClient;
use crate::models::{Query, UserRow, UserResponse, UserInsertRequests};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::{content::RawJson, status::Custom};


#[get("/?<query..>")]
pub async fn get_users(query: Query, client: &rocket::State<ScyllaClient>) -> Custom<RawJson<String>>{

    let result = match query{
        Query{name: Some(name), age: Some(age) } => {
            let prepared_statement = &client.statements.get_user_by_name_age;
            client.session.execute(prepared_statement, (name, age as i32)).await 
        },
        Query{name: Some(name), age: None } => {
            let prepared_statement = &client.statements.get_user_by_name;
            client.session.execute(prepared_statement, (name, )).await
       },
       Query{name: None, age: Some(age) } => {
            let prepared_statement = &client.statements.get_user_by_age;
            client.session.execute(prepared_statement, (age as i32, )).await
       },
       _ => {
           return Custom(Status::InternalServerError, RawJson(json!({"status":"error", "message" : "Something went wrong"}).to_string()));
       }
    };
    if result.is_err() {
        return Custom(
            Status::InternalServerError, 
            RawJson(json!({"status":"error","message": result.err().unwrap().to_string()}).to_string())
        );
    }
    let user_responses : Vec<UserResponse> = result.unwrap().rows.unwrap().into_iter().map(|row| row.into_typed::<UserRow>().unwrap().to_user_response()).collect();
    return Custom(Status::Ok, RawJson(json!({
        "users": user_responses
    }).to_string()))
}


#[post("/", data="<users>", format="json")]
pub async fn save_users(client: &rocket::State<ScyllaClient>, users: Json<UserInsertRequests>) -> Custom<RawJson<String>>{
    let mut batch: Batch = Default::default();
    let user_rows: Vec<UserRow> = users.into_inner().users.into_iter().map(|user_req|{
        batch.append_statement(client.statements.insert_user.clone().as_ref());
        user_req.to_user_row()
    }).collect();
    if let Ok(prepared_batch) = client.session.prepare_batch(&batch).await {
        if let Err(err) = client.session.batch(&prepared_batch, user_rows).await{
            return Custom(Status::InternalServerError, RawJson(json!({"message": err.to_string()}).to_string()));
        }
        return Custom(Status::Ok, RawJson(json!({"status": "ok"}).to_string()));
    }        
    Custom(Status::InternalServerError, RawJson(json!({"status": "not ok"}).to_string()))
}
