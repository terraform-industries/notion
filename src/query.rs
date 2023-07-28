pub trait QueryParams {
    /// Convert the struct to a query string. For example:
    /// ```rust
    /// use notion::models::paging::{Paging, PagingCursor};
    /// use notion::query::QueryParams;
    /// let cursor: PagingCursor = serde_json::from_str("\"foo\"").unwrap();
    /// assert_eq!(
    ///     Paging { start_cursor: Some(cursor), page_size: Some(10) }.to_query_string(),
    ///     "start_cursor=foo&page_size=10"
    /// );
    /// ```
    ///
    /// Note that the query string is not prefixed with a `?`.
    fn to_query_string(&self) -> String;
}
