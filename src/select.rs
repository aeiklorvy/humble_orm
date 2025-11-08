use crate::SqlTable;

fn format_cond<I>(cond: I) -> String
where
    I: IntoIterator<Item = String>,
{
    cond.into_iter()
        .map(|x| format!("({x})"))
        .collect::<Vec<String>>()
        .join(" AND ")
}

/// Builder for `SELECT` statement
///
/// # Example
///
/// ```no_run
/// let sql: String = Select::new()
///     .with_columns([User::NAME, User::AGE])
///     .with_table(User::as_table())
///     .with_where_cond(User::ACTIVE.eq(true))
///     .build();
/// ```
/// Or same in other way:
/// ```no_run
/// let mut select = Select::new();
/// select.push_columns([User::NAME, User::AGE]);
/// select.set_table(User::as_table());
/// select.push_where_cond(User::ACTIVE.eq(true));
/// let sql: String = select.build();
/// ```
#[derive(Clone)]
pub struct Select {
    columns: String,
    table: String,
    cond: Vec<String>,
    group_by: String,
    having: Vec<String>,
    order_by: String,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Select {
    /// Create an empty select
    pub const fn new() -> Self {
        Self {
            columns: String::new(),
            table: String::new(),
            cond: vec![],
            group_by: String::new(),
            having: vec![],
            order_by: String::new(),
            limit: None,
            offset: None,
        }
    }

    /// Adds a column to be selected
    pub fn with_column<T: Into<String>>(mut self, col: T) -> Self {
        self.push_column(col);
        self
    }

    /// Adds columns to be selected
    pub fn with_columns<I>(mut self, cols: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.push_columns(cols);
        self
    }

    /// Sets the table from which the selection will be made
    ///
    /// # Panic
    ///
    /// Panics if the table has already been set earlier. Use joins in this case.
    pub fn with_table<T: SqlTable>(mut self, table: T) -> Self {
        self.set_table(table);
        self
    }

    /// Joins the table for the selection
    ///
    /// # Panic
    ///
    /// Panics if the table has not been set. First set the table as a starting point.
    pub fn with_join<T, I>(mut self, table: T, on: I) -> Self
    where
        T: SqlTable,
        I: IntoIterator<Item = String>,
    {
        self.inner_join(table, on);
        self
    }

    /// Joins the table for the selection
    ///
    /// # Panic
    ///
    /// Panics if the table has not been set. First set the table as a starting point.
    pub fn with_inner_join<T, I>(mut self, table: T, on: I) -> Self
    where
        T: SqlTable,
        I: IntoIterator<Item = String>,
    {
        self.inner_join(table, on);
        self
    }

    /// Joins the table for the selection
    ///
    /// # Panic
    ///
    /// Panics if the table has not been set. First set the table as a starting point.
    pub fn with_left_join<T, I>(mut self, table: T, on: I) -> Self
    where
        T: SqlTable,
        I: IntoIterator<Item = String>,
    {
        self.left_join(table, on);
        self
    }

    /// Adds a selection condition
    pub fn with_where_cond<C: Into<String>>(mut self, cond: C) -> Self {
        self.push_where_cond(cond);
        self
    }

    /// Adds a column to sort the selection
    pub fn with_order<O: Into<String>>(mut self, order: O) -> Self {
        self.push_order(order);
        self
    }

    /// Adds a column to group the selection.
    pub fn with_group<G: Into<String>>(mut self, group: G) -> Self {
        self.push_group(group);
        self
    }

    /// Adds a condition for grouping the selection
    pub fn with_having<H: Into<String>>(mut self, having: H) -> Self {
        self.push_having(having);
        self
    }

    /// Limits the number of rows returned by the query
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.set_limit(limit);
        self
    }

    /// Specifies which line to start receiving data from
    pub fn with_limit_offset(mut self, offset: u32) -> Self {
        self.set_limit_offset(offset);
        self
    }

    /// Adds a column to be selected
    pub fn push_column<T: Into<String>>(&mut self, col: T) {
        if !self.columns.is_empty() {
            self.columns.push(',');
        }
        self.columns += &col.into();
    }

    /// Adds columns to be selected
    pub fn push_columns<I>(&mut self, cols: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        for col in cols {
            self.push_column(col);
        }
    }

    /// Sets the table from which the selection will be made
    ///
    /// # Panic
    ///
    /// Panics if the table has already been set earlier. Use joins in this case.
    pub fn set_table<T: SqlTable>(&mut self, _table: T) {
        #[cfg(debug_assertions)]
        if !self.table.is_empty() {
            panic!("table already exists, use join instead");
        }
        self.table = T::table_name().into();
    }

    /// Joins the table for the selection
    ///
    /// # Panic
    ///
    /// Panics if the table has not been set. First set the table as a starting point.
    pub fn join<T, I>(&mut self, table: T, on: I)
    where
        T: SqlTable,
        I: IntoIterator<Item = String>,
    {
        self.inner_join::<T, I>(table, on);
    }

    /// Joins the table for the selection
    ///
    /// # Panic
    ///
    /// Panics if the table has not been set. First set the table as a starting point.
    pub fn inner_join<T, I>(&mut self, _table: T, on: I)
    where
        T: SqlTable,
        I: IntoIterator<Item = String>,
    {
        #[cfg(debug_assertions)]
        if self.table.is_empty() {
            panic!("join to nothing, use with_table or set_table first");
        }
        // use write to eliminate unnecessary allocations
        use std::fmt::Write;
        let on_cond = format_cond(on);
        write!(self.table, " INNER JOIN {} ON {on_cond}", T::table_name()).unwrap();
    }

    /// Joins the table for the selection
    ///
    /// # Panic
    ///
    /// Panics if the table has not been set. First set the table as a starting point.
    pub fn left_join<T, I>(&mut self, _table: T, on: I)
    where
        T: SqlTable,
        I: IntoIterator<Item = String>,
    {
        #[cfg(debug_assertions)]
        if self.table.is_empty() {
            panic!("join to nothing, use with_table or set_table first");
        }
        // use write to eliminate unnecessary allocations
        use std::fmt::Write;
        let on_cond = format_cond(on);
        write!(self.table, " LEFT JOIN {} ON {on_cond}", T::table_name()).unwrap();
    }

    /// Adds a selection condition
    pub fn push_where_cond<C: Into<String>>(&mut self, cond: C) {
        self.cond.push(cond.into());
    }

    /// Adds a column to sort the selection
    pub fn push_order<O: Into<String>>(&mut self, order: O) {
        if !self.order_by.is_empty() {
            self.order_by.push(',');
        }
        self.order_by += &order.into();
    }

    /// Adds a column to group the selection.
    pub fn push_group<G: Into<String>>(&mut self, group: G) {
        if !self.group_by.is_empty() {
            self.group_by.push(',');
        }
        self.group_by += &group.into();
    }

    /// Adds a condition for grouping the selection
    pub fn push_having<H: Into<String>>(&mut self, having: H) {
        self.having.push(having.into());
    }

    /// Limits the number of rows returned by the query
    pub fn set_limit(&mut self, limit: u32) {
        self.limit = Some(limit);
    }

    /// Specifies which line to start receiving data from
    pub fn set_limit_offset(&mut self, offset: u32) {
        self.offset = Some(offset);
    }

    /// Performs query building by consuming itself
    pub fn build(self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.columns, self.table);
        if !self.cond.is_empty() {
            sql += " WHERE ";
            sql += &format_cond(self.cond);
        }
        if !self.group_by.is_empty() {
            sql += " GROUP BY ";
            sql += &self.group_by;
        }
        if !self.having.is_empty() {
            sql += " HAVING ";
            sql += &format_cond(self.having);
        }
        if !self.order_by.is_empty() {
            sql += " ORDER BY ";
            sql += &self.order_by;
        }
        if let Some(limit) = self.limit {
            // use write to eliminate unnecessary allocations
            use std::fmt::Write;
            write!(sql, " LIMIT {limit}").unwrap();
            if let Some(offset) = self.offset {
                write!(sql, " OFFSET {offset}").unwrap();
            }
        }
        sql
    }
}
