use scylla::{Session, SessionBuilder, prepared_statement::PreparedStatement};

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
        let uri = "127.0.0.1:9042".to_string();
        let session = SessionBuilder::new()
            .known_node(uri)
            .build()
            .await?;
        Self::create_keyspace(&session).await?;
        Self::create_users_table(&session).await?;
        Self::create_indexes(&session).await?;

        let mut get_user_by_name = session.prepare("SELECT * FROM project.users WHERE name = ?").await?;
        get_user_by_name.set_page_size(10);
        let mut get_user_by_age = session.prepare("SELECT * FROM project.users WHERE age = ?").await?;
        get_user_by_age.set_page_size(10);
        let mut get_user_by_name_age = session.prepare("SELECT * FROM project.users WHERE name = ? AND age = ? ALLOW FILTERING").await?;
        get_user_by_name_age.set_page_size(10);

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
