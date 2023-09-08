use crate::ids::UserId;
use monostate::MustBe;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    pub id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl Serialize for UserCommon {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UserCommon", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("avatar_url", &self.avatar_url)?;
        state.serialize_field("object", "user")?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum User {
    Person {
        #[serde(flatten)]
        common: UserCommon,
        #[serde(skip_serializing_if = "Option::is_none")]
        person: Option<Person>,
        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<MustBe!("person")>,
    },
    Bot {
        #[serde(flatten)]
        common: UserCommon,
        #[serde(skip_serializing_if = "Option::is_none")]
        bot: Option<Bot>,
        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<MustBe!("bot")>,
    },
}

#[cfg(test)]
mod test {
    use super::User;

    #[test]
    fn parse_user() {
        let a = r#"{
                "object": "user",
                "id": "00000000-0000-0000-0000-000000000000"
            }"#;
        let b = r#"{
                "object": "user",
                "id": "00000000-0000-0000-0000-000000000000",
                "type": "person"
            }"#;
        let c = r#"{
                "object": "user",
                "id": "00000000-0000-0000-0000-000000000000",
                "type": "bot"
            }"#;
        let d = r#"{
            "object": "user",
            "id": "00000000-0000-0000-0000-000000000000",
            "type": "person",
            "person": {
                "email": "foo@example.org"
            }
        }"#;
        let e = r#"{
            "object": "user",
            "id": "00000000-0000-0000-0000-000000000000",
            "person": {
                "email": "foo@example.org"
            }
        }"#;
        let f = r#"{
            "object": "user",
            "id": "00000000-0000-0000-0000-000000000000",
            "type": "bot",
            "bot": {
                "workspace_name": "My Workspace"
            }
        }"#;
        let g = r#"{
            "object": "user",
            "id": "00000000-0000-0000-0000-000000000000",
            "bot": {
                "workspace_name": "My Workspace"
            }
        }"#;

        serde_json::from_str::<User>(a).unwrap();
        serde_json::from_str::<User>(b).unwrap();
        serde_json::from_str::<User>(c).unwrap();
        serde_json::from_str::<User>(d).unwrap();
        serde_json::from_str::<User>(e).unwrap();
        serde_json::from_str::<User>(f).unwrap();
        serde_json::from_str::<User>(g).unwrap();
    }
}
