
#[derive(FromForm)]
pub struct Query{
    pub name: Option<String>,
    pub age: Option<u32>
}

