use cdrs_tokio::frame::TryFromRow;

#[derive(Clone, Debug, PartialEq)]
pub struct ScyllaCount(i64);

impl ScyllaCount {
    pub fn into_inner(self) -> i64 {
        self.0
    }
}

impl TryFromRow for ScyllaCount {
    fn try_from_row(row: cdrs_tokio::types::rows::Row) -> cdrs_tokio::error::Result<ScyllaCount> {
        let count = match cdrs_tokio::types::IntoRustByName::<i64>::get_by_name(&row, "count") {
            Ok(Some(val)) => val,
            _ => {
                return Err(cdrs_tokio::error::Error::from(
                    "Failed to get 'count' from row",
                ))
            }
        };

        Ok(ScyllaCount(count))
    }
}
