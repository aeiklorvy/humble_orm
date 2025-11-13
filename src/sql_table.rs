use crate::SqlColumn;

/// Represents any type that can be considered as an SQL table
pub trait SqlTable: Default {
    /// The name of the table (for building queries)
    ///
    /// It should be an already escaped string like `"\"User\""`.
    const TABLE_NAME: &'static str;

    /// An array of all the columns in the table
    const COLUMNS: &'static [SqlColumn];

    /// Returns a table as an Entity (for building queries)
    fn as_table() -> Self {
        // All generated structs are both an Entity and a Model. We only need a
        // type from an Entity, so the data in Model can be absolutely
        // anything (not used).
        Self::default()
    }

    /// Checks if there is a column with the specified name in the table
    fn has_column(name: &str) -> bool {
        Self::COLUMNS
            .iter()
            .find(|col| col.name() == name)
            .is_some()
    }

    /// Returns the index of the column with the specified name in the table
    fn column_index(name: &str) -> Option<usize> {
        Self::COLUMNS
            .iter()
            .enumerate()
            .find(|(_, col)| col.name() == name)
            .map(|(i, _)| i)
    }

    /// Returns the name of a column in a table by its index
    fn column_name_at(index: usize) -> Option<&'static str> {
        Self::COLUMNS.get(index).map(|col| col.name())
    }
}
