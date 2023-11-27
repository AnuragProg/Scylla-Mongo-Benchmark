use mongodb::{Client, Database, Collection, options::{FindOptions, ClientOptions}, bson::doc, IndexModel};
use crate::models::{UserDoc, UserInsertRequests, UserResponse};
use crate::repository::UserRepository;


pub struct MongoClient{
    session: Client,
    db: Database,
    users_collection: Collection<UserDoc>
}



impl MongoClient{
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let uri = std::env::var("MONGO_URI").unwrap_or_else(|_|"mongodb://localhost:27017".to_string());
        let options = ClientOptions::parse(uri).await?;
        let session = Client::with_options(options)?;
        let db = session.database("benchmark");
        let users_collection = db.collection("users");
        let name_index = IndexModel::builder()
            .keys(doc!{"name": 1})
            .build();
        let age_index = IndexModel::builder()
            .keys(doc!{"age":1})
            .build();
        users_collection.create_indexes(vec![name_index, age_index], None).await?;
        Ok(Self{ session, db, users_collection})
    }
}


#[async_trait]
impl UserRepository for MongoClient{
    async fn insert_users(&self, user_insert_requests: UserInsertRequests) -> Result<(), Box<dyn std::error::Error>>{
        let docs = user_insert_requests.to_user_docs();
        self.users_collection.insert_many(docs, None).await?;
        Ok(())
    }
    async fn get_users_by_name(&self, name: String, _next_page_token: Option<String>) -> Result<UserResponse, Box<dyn std::error::Error>>{
        let options = FindOptions::builder().limit(100).build();
        let mut cursor = self.users_collection.find(doc!{"name": name}, options).await?;
        let mut count = 0;
        let mut docs = Vec::new();
        while cursor.advance().await? && count < 100{
            docs.push(cursor.deserialize_current()?.to_user_row());
            count += 1;
        }
        Ok(UserResponse{ users: docs, next_page_token: None })
    }
    async fn get_users_by_age(&self, age: u32, _next_page_token: Option<String>) -> Result<UserResponse, Box<dyn std::error::Error>>{
        let options = FindOptions::builder().limit(100).build();
        let mut cursor = self.users_collection.find(doc!{"age": age}, options).await?;
        let mut count = 0;
        let mut docs = Vec::new();
        while cursor.advance().await? && count < 100{
            docs.push(cursor.deserialize_current()?.to_user_row());
            count += 1;
        }
        Ok(UserResponse{ users: docs, next_page_token: None })
    }
    async fn get_users_by_name_age(&self, name: String, age: u32, next_page_token: Option<String>)  -> Result<UserResponse, Box<dyn std::error::Error>>{
        let options = FindOptions::builder().limit(100).build();
        let mut cursor = self.users_collection.find(doc!{"name": name, "age": age}, options).await?;
        let mut count = 0;
        let mut docs = Vec::new();
        while cursor.advance().await? && count < 100{
            docs.push(cursor.deserialize_current()?.to_user_row());
            count += 1;
        }
        Ok(UserResponse{ users: docs, next_page_token: None })
    }
}
