use crate::SqlValue;

/// Information about the column entity for working with SQL
#[derive(Clone, Copy)]
pub struct SqlColumn {
    name: &'static str,
    table_name: &'static str,
    is_primary: bool,
}

impl std::fmt::Display for SqlColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // expect they are escaped
        write!(f, "{}.{}", self.table_name, self.name)
    }
}

impl From<SqlColumn> for String {
    fn from(value: SqlColumn) -> Self {
        value.to_string()
    }
}

impl SqlValue for SqlColumn {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlColumn {
    /// Creates a new sql column
    ///
    /// # Safety
    ///
    /// The function relies on the passed values to be properly escaped.
    /// Otherwise, the constructed queries may be incorrect due to the
    /// matching of names with SQL keywords. An example of the correct use of
    /// the function:
    ///
    /// ```no_run
    /// let user_id_col = SqlColumn::new("\"id\"", "\"User\"", true);
    /// let user_name_col = SqlColumn::new("\"name\"", "\"User\"", false);
    /// ```
    pub const unsafe fn new(
        name: &'static str,
        table_name: &'static str,
        is_primary: bool,
    ) -> Self {
        Self {
            name,
            table_name,
            is_primary,
        }
    }

    /// Returns name of the column
    pub const fn name(&self) -> &'static str {
        trim_quotes(self.name)
    }

    /// Returns name of the table
    pub const fn table_name(&self) -> &'static str {
        trim_quotes(self.table_name)
    }

    /// Returns `true` if a primary key is defined for the column
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// Produces an alias: `{field} AS {alias}`
    pub fn alias(self, alias: &str) -> String {
        format!("{self} AS {alias:?}")
    }

    /// Produces `COUNT({field})`
    pub fn count(self) -> String {
        format!("COUNT({self})")
    }

    /// Produces `COUNT({field}) AS {alias}`
    pub fn count_as(self, alias: &str) -> String {
        format!("COUNT({self}) AS {alias:?}")
    }

    /// Produces `SUM({field})`
    pub fn sum(self) -> String {
        format!("SUM({self})")
    }

    /// Produces `SUM({field}) AS {alias}`
    pub fn sum_as(self, alias: &str) -> String {
        format!("SUM({self}) AS {alias:?}")
    }

    /// Produces `AVG({field})`
    pub fn avg(self) -> String {
        format!("AVG({self})")
    }

    /// Produces `AVG({field}) AS {alias}`
    pub fn avg_as(self, alias: &str) -> String {
        format!("AVG({self}) AS {alias}")
    }

    /// Produces `MIN({field})`
    pub fn min(self) -> String {
        format!("MIN({self})")
    }

    /// Produces `MIN({field}) AS {alias}`
    pub fn min_as(self, alias: &str) -> String {
        format!("MIN({self}) AS {alias}")
    }

    /// Produces `MAX({field})`
    pub fn max(self) -> String {
        format!("MAX({self})")
    }

    /// Produces `MAX({field}) AS {alias}`
    pub fn max_as(self, alias: &str) -> String {
        format!("MAX({self}) AS {alias}")
    }

    /// Produces `{field} ASC` (for ordering)
    pub fn asc(self) -> String {
        format!("{self} ASC")
    }

    /// Produces `{field} DESC` (for ordering)
    pub fn desc(self) -> String {
        format!("{self} DESC")
    }

    /// Produces `{field} IS NULL`
    pub fn is_null(self) -> String {
        format!("{self} IS NULL")
    }

    /// Produces `{field} IS NOT NULL`
    pub fn is_not_null(self) -> String {
        format!("{self} IS NOT NULL")
    }

    /// Produces `A = B`
    pub fn eq<V: SqlValue>(self, value: V) -> String {
        format!("{self} = {}", value.to_sql())
    }

    /// Produces `A != B`
    pub fn ne<V: SqlValue>(self, value: V) -> String {
        format!("{self} != {}", value.to_sql())
    }

    /// Produces `A > B`
    pub fn gt<V: SqlValue>(self, value: V) -> String {
        format!("{self} > {}", value.to_sql())
    }

    /// Produces `A >= B`
    pub fn ge<V: SqlValue>(self, value: V) -> String {
        format!("{self} >= {}", value.to_sql())
    }

    /// Produces `A < B`
    pub fn lt<V: SqlValue>(self, value: V) -> String {
        format!("{self} < {}", value.to_sql())
    }

    /// Produces `A <= B`
    pub fn le<V: SqlValue>(self, value: V) -> String {
        format!("{self} <= {}", value.to_sql())
    }

    /// Produces `A LIKE B`
    pub fn like<V: SqlValue>(self, value: V) -> String {
        format!("{self} LIKE {}", value.to_sql())
    }

    /// Produces `A NOT LIKE B`
    pub fn not_like<V: SqlValue>(self, value: V) -> String {
        format!("{self} NOT LIKE {}", value.to_sql())
    }

    /// Produces `A IN (...)`
    pub fn in_list<I>(self, values: I) -> String
    where
        I: IntoIterator,
        I::Item: SqlValue,
    {
        let tmp: Vec<String> = values.into_iter().map(|val| val.to_sql()).collect();
        if tmp.is_empty() {
            String::from("false")
        } else {
            format!("{self} IN ({})", tmp.join(","))
        }
    }

    /// Produces `A NOT IN (...)`
    pub fn not_in_list<I>(self, values: I) -> String
    where
        I: IntoIterator,
        I::Item: SqlValue,
    {
        let tmp: Vec<String> = values.into_iter().map(|val| val.to_sql()).collect();
        if tmp.is_empty() {
            String::from("true")
        } else {
            format!("{self} NOT IN ({})", tmp.join(","))
        }
    }

    /// Produces `A BETWEEN (B) AND (C)`
    pub fn between<L, R>(self, left: L, right: R) -> String
    where
        L: SqlValue,
        R: SqlValue,
    {
        format!(
            "{self} BETWEEN ({}) AND ({})",
            left.to_sql(),
            right.to_sql()
        )
    }
}

const fn trim_quotes(s: &'static str) -> &'static str {
    // all these complexities are needed to make the function `const`

    // convert to a byte slice, because we don't know how
    // to work with strings in `const`
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len >= 2 {
        let first = bytes[0];
        let last = bytes[len - 1];
        match (first, last) {
            (b'\"', b'\"') | (b'\'', b'\'') => {
                unsafe {
                    // SAFETY:
                    //
                    // because of: `Index` is not yet stable as a const trait
                    // at the moment we can't just do that:
                    // let new_slice = &bytes[1 .. len-1];
                    //
                    // We'll have to tinker with the raw pointers, and that's
                    // the plan of action:
                    // - delete the first quotation mark by increasing the
                    //   pointer to the beginning of the slice by one.
                    // - remove the second quotation mark, reducing the size
                    //   of the slice by one
                    //
                    // The cut-off quotes are part of a static string, so
                    // memory leaks are excluded. And we know for sure that
                    // the slice length allows us to reduce its size by 2
                    // bytes.
                    let new_slice = std::slice::from_raw_parts(bytes.as_ptr().add(1), len - 1);
                    std::str::from_utf8_unchecked(new_slice)
                }
            }
            _ => s, // there are no quotes in the string
        }
    } else {
        s // length of the string does not imply the presence of quotation marks
    }
}
