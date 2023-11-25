use scylla::{ValueList, FromRow};
use serde::{Serialize, Deserialize };
use serde::ser::{SerializeStruct, Serializer};
use serde::de::{Visitor, Deserializer};
use uuid::Uuid;


#[derive(ValueList, FromRow, Debug)]
pub struct UserRow{
    pub id: Uuid,
    pub name: String,
    pub age: i32
}

impl Serialize for UserRow{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer{
            let mut state = serializer.serialize_struct("UserRow", 3)?;
            state.serialize_field("id", &self.id.to_string())?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("age", &self.age)?;
            state.end()
        }
}
struct UserRowVisitor;
impl<'de> Visitor<'de> for UserRowVisitor{
    type Value = UserRow;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct UserRow")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
where
        A: serde::de::SeqAccess<'de>,
    {
        let id: String = seq.next_element()?.unwrap();
        let name: String = seq.next_element()?.unwrap();
        let age: i32 = seq.next_element()?.unwrap();

        Ok(UserRow{
            id: Uuid::parse_str(&id).unwrap(),
            name,
            age
        })
    }
}
impl<'de> Deserialize<'de> for UserRow{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        deserializer.deserialize_struct("UserRow", &["id", "name", "age"], UserRowVisitor)
    }
}

/*** Below will be communicated with clients through wire ****/
//impl UserRow{
//    pub fn to_user_response(self) -> UserResponse{
//        UserResponse{
//            id: self.id.to_string(),
//            name: self.name,
//            age: self.age,
//        }
//    }
//}

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
    pub users: Vec<UserRow>,
    pub next_page_token: Option<String>
}

