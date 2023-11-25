
#[derive(FromForm)]
pub struct UserQuery{
    pub name: Option<String>,
    pub age: Option<u32>,
    pub next_page_token: Option<String>
}

