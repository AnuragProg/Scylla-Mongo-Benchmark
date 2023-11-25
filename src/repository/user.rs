use async_trait::async_trait;
use crate::models::{UserInsertRequests, UserResponse};

#[async_trait]
pub trait UserRepository : Send + Sync{
    async fn insert_users(&self, user_insert_requests: UserInsertRequests) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_users_by_name(&self, name: String, next_page_token: Option<String>) -> Result<UserResponse, Box<dyn std::error::Error>>;
    async fn get_users_by_age(&self, age: u32, next_page_token: Option<String>) -> Result<UserResponse, Box<dyn std::error::Error>>;
    async fn get_users_by_name_age(&self, name: String, age: u32, next_page_token: Option<String>)  -> Result<UserResponse, Box<dyn std::error::Error>>;
}
