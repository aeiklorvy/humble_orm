/// Represents any value that can be considered as an SQL table
pub trait SqlTable: Default {
    /// The name of the table (for building queries)
    ///
    /// It should return an already escaped string like `"\"User\""`.
    fn table_name() -> &'static str;

    /// Returns a table as an Entity (for building queries)
    fn as_table() -> Self {
        // All generated structs are both an Entity and a Model. We only need a
        // type from an Entity, so the data in Model can be absolutely
        // anything (not used).
        Self::default()
    }
}
