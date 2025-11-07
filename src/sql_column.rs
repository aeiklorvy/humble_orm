use crate::SqlValue;

/// Information about the column entity for working with SQL
#[derive(Clone, Copy)]
pub struct SqlColumn {
    name: &'static str,
    table_name: &'static str,
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
    pub const fn new(name: &'static str, table_name: &'static str) -> Self {
        Self { name, table_name }
    }

    /// returns name of the column
    pub fn name(&self) -> &str {
        self.name
            .strip_suffix('\"')
            .unwrap_or(self.name)
            .strip_prefix('\"')
            .unwrap_or(self.name)
    }

    /// produces an alias: `{field} AS {alias}`
    pub fn alias(self, alias: &str) -> String {
        format!("{self} AS {alias:?}")
    }

    /// produces `COUNT({field})`
    pub fn count(self) -> String {
        format!("COUNT({self})")
    }

    /// produces `COUNT({field}) AS {alias}`
    pub fn count_as(self, alias: &str) -> String {
        format!("COUNT({self}) AS {alias:?}")
    }

    /// produces `SUM({field})`
    pub fn sum(self) -> String {
        format!("SUM({self})")
    }

    /// produces `SUM({field}) AS {alias}`
    pub fn sum_as(self, alias: &str) -> String {
        format!("SUM({self}) AS {alias:?}")
    }

    /// produces `AVG({field})`
    pub fn avg(self) -> String {
        format!("AVG({self})")
    }

    /// produces `AVG({field}) AS {alias}`
    pub fn avg_as(self, alias: &str) -> String {
        format!("AVG({self}) AS {alias}")
    }

    /// produces `MIN({field})`
    pub fn min(self) -> String {
        format!("MIN({self})")
    }

    /// produces `MIN({field}) AS {alias}`
    pub fn min_as(self, alias: &str) -> String {
        format!("MIN({self}) AS {alias}")
    }

    /// produces `MAX({field})`
    pub fn max(self) -> String {
        format!("MAX({self})")
    }

    /// produces `MAX({field}) AS {alias}`
    pub fn max_as(self, alias: &str) -> String {
        format!("MAX({self}) AS {alias}")
    }

    /// produces `{field} IS NULL`
    pub fn is_null(self) -> String {
        format!("{self} IS NULL")
    }

    /// produces `{field} IS NOT NULL`
    pub fn is_not_null(self) -> String {
        format!("{self} IS NOT NULL")
    }

    /// produces `A = B`
    pub fn eq<V: SqlValue>(self, value: V) -> String {
        format!("{self} = {}", value.to_sql())
    }

    /// produces `A != B`
    pub fn ne<V: SqlValue>(self, value: V) -> String {
        format!("{self} != {}", value.to_sql())
    }

    /// produces `A > B`
    pub fn gt<V: SqlValue>(self, value: V) -> String {
        format!("{self} > {}", value.to_sql())
    }

    /// produces `A >= B`
    pub fn ge<V: SqlValue>(self, value: V) -> String {
        format!("{self} >= {}", value.to_sql())
    }

    /// produces `A < B`
    pub fn lt<V: SqlValue>(self, value: V) -> String {
        format!("{self} < {}", value.to_sql())
    }

    /// produces `A <= B`
    pub fn le<V: SqlValue>(self, value: V) -> String {
        format!("{self} <= {}", value.to_sql())
    }

    /// produces `A LIKE B`
    pub fn like<V: SqlValue>(self, value: V) -> String {
        format!("{self} LIKE {}", value.to_sql())
    }

    /// produces `A NOT LIKE B`
    pub fn not_like<V: SqlValue>(self, value: V) -> String {
        format!("{self} NOT LIKE {}", value.to_sql())
    }

    /// produces `A IN (...)`
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

    /// produces `A NOT IN (...)`
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

    /// produces `A BETWEEN (B) AND (C)`
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
