use scylla::{ValueList, FromRow};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
//use rocket::serde::{Serialize, Deserialize};

#[derive(ValueList, FromRow)]
pub struct UserRow{
    pub id: Uuid,
    pub name: String,
    pub age: i32
}

/*** Below will be communicated with clients through wire ****/
impl UserRow{
    pub fn to_user_response(self) -> UserResponse{
        UserResponse{
            id: self.id.to_string(),
            name: self.name,
            age: self.age,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInsertRequest{
    pub name: String,
    pub age: i32
}

impl UserInsertRequest{
    pub fn to_user_row(self) -> UserRow{
        UserRow{
            id: uuid::Uuid::new_v4(),
            name: self.name,
            age: self.age,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInsertRequests{
    pub users: Vec<UserInsertRequest>
}

impl UserInsertRequests{
    pub fn to_user_rows(self) -> Vec<UserRow>{
        self.users.into_iter().map(|user_row| user_row.to_user_row()).collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse{
    pub id: String,
    pub name: String,
    pub age: i32
}

#[derive(Serialize, Deserialize)]
pub struct UserResponses{
    pub users: Vec<UserResponse>,
    pub next_page_token: String
}

