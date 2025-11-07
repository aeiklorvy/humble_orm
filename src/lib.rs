#![doc = include_str!("../README.md")]

mod select;
mod sql_column;
mod sql_table;
mod sql_value;

pub use select::Select;
pub use sql_column::SqlColumn;
pub use sql_table::SqlTable;
pub use sql_value::SqlValue;

pub use humble_orm_macro::*;

/// produces `[A, B, C] → (A) AND (B) AND (C)`
///
/// # Example
///
/// ```no_run
/// let cond = join_and([User::NAME.eq("John"), User::AGE.gt(30)]);
/// assert_eq!(cond, r#"("User"."name" = "John") AND ("User"."age" > 30)"#)
/// ```
pub fn join_and<I>(cond: I) -> String
where
    I: IntoIterator<Item = String>,
{
    cond.into_iter()
        .map(|x| format!("({x})"))
        .collect::<Vec<String>>()
        .join(" AND ")
}

/// produces `[A, B, C] → (A) OR (B) OR (C)`
///
/// # Example
///
/// ```no_run
/// let cond = join_or([User::NAME.eq("John"), User::NAME.eq("Jack")]);
/// assert_eq!(cond, r#"("User"."name" = "John") OR ("User"."name" = "Jack")"#)
/// ```
pub fn join_or<I>(cond: I) -> String
where
    I: IntoIterator<Item = String>,
{
    cond.into_iter()
        .map(|x| format!("({x})"))
        .collect::<Vec<String>>()
        .join(" OR ")
}
