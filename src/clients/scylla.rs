use scylla::{Session, SessionBuilder, prepared_statement::PreparedStatement, batch::Batch, query::Query};
use crate::repository::UserRepository;
use crate::models::{UserInsertRequests, UserResponse, UserRow};

pub struct Statements{
    pub insert_user: String,
    pub get_user_by_name: PreparedStatement,
    pub get_user_by_age: PreparedStatement,
    pub get_user_by_name_age: PreparedStatement,
}

pub struct ScyllaClient{
    pub session: Session,
    pub statements: Statements
}

impl ScyllaClient{
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_|"127.0.0.1:9042".to_string());
        let session = SessionBuilder::new()
            .known_node(uri)
            .build()
            .await?;
        Self::create_keyspace(&session).await?;
        Self::create_users_table(&session).await?;
        Self::create_indexes(&session).await?;

        let get_user_by_name = session.prepare(
            Query::new("SELECT * FROM project.users WHERE name = ?").with_page_size(100)
        ).await?;
        let get_user_by_age = session.prepare(
            Query::new("SELECT * FROM project.users WHERE age = ?").with_page_size(100)
        ).await?;
        let get_user_by_name_age = session.prepare(
            Query::new("SELECT * FROM project.users WHERE name = ? AND age = ? ALLOW FILTERING").with_page_size(100)
        ).await?;

        Ok(Self{
            session,
            statements: Statements{
                insert_user: "INSERT INTO project.users(id, name, age) VALUES(?, ?, ?);".to_string(),
                get_user_by_name,
                get_user_by_age,
                get_user_by_name_age
            }
        })
    }

    async fn create_keyspace(session: &Session) -> Result<scylla::QueryResult, scylla::transport::errors::QueryError> {
        session.query("
            CREATE KEYSPACE IF NOT EXISTS project WITH REPLICATION = {'class':'NetworkTopologyStrategy', 'replication_factor': 1};
        ", &[]).await
    }

    async fn create_users_table(session: &Session) -> Result<scylla::QueryResult, scylla::transport::errors::QueryError> {
        session.query("
            CREATE TABLE IF NOT EXISTS project.users(
                id uuid,
                name text,
                age int,
                PRIMARY KEY((id), name, age)
            );
        ", &[]).await
    }


    async fn create_indexes(session: &Session)-> Result<(), scylla::transport::errors::QueryError> {
        session.query("
            CREATE INDEX IF NOT EXISTS users_by_name ON project.users(name);
        ", &[]).await?;

        session.query("
            CREATE INDEX IF NOT EXISTS users_by_age ON project.users(age);
        ", &[]).await?;

        Ok(())
    }
}


#[async_trait]
impl UserRepository for ScyllaClient{
    async fn insert_users(&self, user_insert_requests: UserInsertRequests) -> Result<(), Box<dyn std::error::Error>>{
        let mut batch: Batch = Default::default();
        let user_rows: Vec<UserRow> = user_insert_requests.users.into_iter().map(|user_req|{
            batch.append_statement(self.statements.insert_user.clone().as_ref());
            user_req.to_user_row()
        }).collect();
        let prepared_batch = self.session.prepare_batch(&batch).await?;
        self.session.batch(&prepared_batch, user_rows).await?;
        Ok(())
    }
    async fn get_users_by_name(&self, name: String, next_page_token: Option<String>) -> Result<UserResponse, Box<dyn std::error::Error>>{
        let prepared_statement = &self.statements.get_user_by_name;
        let result = match next_page_token { 
            Some(token) => self.session.execute_paged(prepared_statement, (name, ), Some(bytes::Bytes::from(token))).await?,
            None => self.session.execute(prepared_statement, (name, )).await?,
        };
        let user_rows : Vec<UserRow> = result.rows.unwrap().into_iter().map(|row| row.into_typed::<UserRow>().unwrap()).collect();
        Ok(UserResponse{
            users: user_rows,
            next_page_token: None
        })
    }
    async fn get_users_by_age(&self, age: u32, next_page_token: Option<String>) -> Result<UserResponse, Box<dyn std::error::Error>>{
        let prepared_statement = &self.statements.get_user_by_age;
        let result = match next_page_token {
            Some(token) => self.session.execute_paged(prepared_statement, (age as i32, ), Some(bytes::Bytes::from(token))).await?,
            None => self.session.execute(prepared_statement, (age as i32, )).await?,
        };
        let user_rows : Vec<UserRow> = result.rows.unwrap().into_iter().map(|row| row.into_typed::<UserRow>().unwrap()).collect();
        println!("{:?}", result.paging_state.clone().unwrap().to_vec());
        Ok(UserResponse{
            users: user_rows,
            next_page_token: None
        })
    }
    async fn get_users_by_name_age(&self, name: String, age: u32, next_page_token: Option<String>)  -> Result<UserResponse, Box<dyn std::error::Error>>{
        let prepared_statement = &self.statements.get_user_by_name_age;
        let result = match next_page_token {
            Some(token) => self.session.execute_paged(prepared_statement, (name, age as i32 ), Some(bytes::Bytes::from(token))).await?,
            None => self.session.execute(prepared_statement, (name, age as i32 )).await?,
        };

        let user_rows : Vec<UserRow> = result.rows.unwrap().into_iter().map(|row| row.into_typed::<UserRow>().unwrap()).collect();
        Ok(UserResponse{
            users: user_rows,
            next_page_token: None
        })
    }
}
