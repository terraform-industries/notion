pub mod block;
pub mod error;
pub mod paging;
pub mod properties;
pub mod property_schema;
pub mod search;
#[cfg(test)]
mod tests;
pub mod text;
pub mod users;

use crate::models::properties::{PropertyConfiguration, PropertyValue};
use crate::models::text::RichText;
use crate::Error;
use block::ExternalFileObject;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ids::{AsIdentifier, BlockId, DatabaseId, PageId};
use crate::models::block::{Block, CreateBlock};
use crate::models::error::ErrorResponse;
use crate::models::paging::PagingCursor;
use crate::models::users::User;
pub use chrono::{DateTime, Utc};
pub use serde_json::value::Number;

use self::property_schema::PropertySchema;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum ObjectType {
    Database,
    List,
}

/// Represents a Notion Database
/// See <https://developers.notion.com/reference/database>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Database {
    /// Unique identifier for the database.
    pub id: DatabaseId,
    /// Date and time when this database was created.
    pub created_time: DateTime<Utc>,
    /// Date and time when this database was updated.
    pub last_edited_time: DateTime<Utc>,
    /// Name of the database as it appears in Notion.
    pub title: Vec<RichText>,
    /// Schema of properties for the database as they appear in Notion.
    //
    // key string
    // The name of the property as it appears in Notion.
    //
    // value object
    // A Property object.
    pub icon: Option<IconObject>,
    pub properties: HashMap<String, PropertyConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct InternalFileDetails {
    pub url: String,
    pub expiry_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum IconObject {
    File {
        file: InternalFileDetails,
    },
    External {
        external: ExternalFileObject,
    },
    Emoji {
        emoji: String,
    },
}

impl AsIdentifier<DatabaseId> for Database {
    fn as_id(&self) -> &DatabaseId {
        &self.id
    }
}

impl Database {
    pub fn title_plain_text(&self) -> String {
        self.title
            .iter()
            .flat_map(|rich_text| rich_text.plain_text().chars())
            .collect()
    }
}

/// <https://developers.notion.com/reference/pagination#responses>
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub next_cursor: Option<PagingCursor>,
    pub has_more: bool,
    /// Type of response for query/search results
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    /// Additional metadata for page_or_database queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_or_database: Option<serde_json::Value>,
    /// Request identifier for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl<T> ListResponse<T> {
    pub fn results(&self) -> &[T] {
        &self.results
    }
}

impl ListResponse<Object> {
    pub fn only_databases(self) -> ListResponse<Database> {
        let databases = self
            .results
            .into_iter()
            .filter_map(|object| match object {
                Object::Database { database } => Some(database),
                _ => None,
            })
            .collect();

        ListResponse {
            results: databases,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
            response_type: self.response_type,
            page_or_database: self.page_or_database,
            request_id: self.request_id,
        }
    }

    pub(crate) fn expect_databases(self) -> Result<ListResponse<Database>, crate::Error> {
        let databases: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Database { database } => Ok(database),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: databases?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
            response_type: self.response_type,
            page_or_database: self.page_or_database,
            request_id: self.request_id,
        })
    }

    pub(crate) fn expect_pages(self) -> Result<ListResponse<Page>, crate::Error> {
        let items: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Page { page } => Ok(page),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: items?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
            response_type: self.response_type,
            page_or_database: self.page_or_database,
            request_id: self.request_id,
        })
    }

    pub(crate) fn expect_blocks(self) -> Result<ListResponse<Block>, crate::Error> {
        let items: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Block { block } => Ok(block),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: items?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
            response_type: self.response_type,
            page_or_database: self.page_or_database,
            request_id: self.request_id,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Parent {
    #[serde(rename = "database_id")]
    Database { database_id: DatabaseId },
    #[serde(rename = "page_id")]
    Page { page_id: PageId },
    Workspace {
        /// Always `true`
        workspace: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Properties {
    #[serde(flatten)]
    pub properties: HashMap<String, PropertyValue>,
}

impl Properties {
    pub fn title(&self) -> Option<String> {
        self.properties.values().find_map(|p| match p {
            PropertyValue::Title { title, .. } => {
                Some(title.iter().map(|t| t.plain_text()).collect())
            }
            _ => None,
        })
    }
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct BlockAppendChildrenRequest {
    pub children: Vec<CreateBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<BlockId>,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct PageCreateRequest {
    pub parent: Parent,
    pub properties: Properties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<CreateBlock>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub id: PageId,
    /// Date and time when this page was created.
    pub created_time: DateTime<Utc>,
    /// Date and time when this page was updated.
    pub last_edited_time: DateTime<Utc>,
    /// User who created the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<User>,
    /// User who last edited the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_by: Option<User>,
    /// Cover image for the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<IconObject>,
    /// The archived status of the page.
    pub archived: bool,
    /// Whether the page is in trash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_trash: Option<bool>,
    pub properties: Properties,
    pub icon: Option<IconObject>,
    pub parent: Parent,
    pub url: String,
    pub public_url: Option<String>,
}

impl Page {
    pub fn title(&self) -> Option<String> {
        self.properties.title()
    }
}

impl AsIdentifier<PageId> for Page {
    fn as_id(&self) -> &PageId {
        &self.id
    }
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct DatabaseCreateRequest {
    pub parent: Parent,
    pub properties: HashMap<String, PropertySchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<RichText>>,
}

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "object")]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Block {
        #[serde(flatten)]
        block: Block,
    },
    Database {
        #[serde(flatten)]
        database: Database,
    },
    Page {
        #[serde(flatten)]
        page: Page,
    },
    List {
        #[serde(flatten)]
        list: ListResponse<Object>,
    },
    User {
        #[serde(flatten)]
        user: User,
    },
    Error {
        #[serde(flatten)]
        error: ErrorResponse,
    },
}

impl Object {
    pub fn is_database(&self) -> bool {
        matches!(self, Object::Database { .. })
    }
}
