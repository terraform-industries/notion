use crate::query::QueryParams;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct PagingCursor(String);

#[derive(Serialize, Debug, Eq, PartialEq, Default, Clone)]
pub struct Paging {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<PagingCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,
}

pub trait Pageable {
    fn start_from(
        self,
        starting_point: Option<PagingCursor>,
    ) -> Self;
}

impl QueryParams for Paging {
    fn to_query_string(&self) -> String {
        let mut params = vec![];
        if let Some(start_cursor) = &self.start_cursor {
            params.push(format!("start_cursor={}", start_cursor.0));
        }
        if let Some(page_size) = &self.page_size {
            params.push(format!("page_size={}", page_size));
        }
        params.join("&")
    }
}
