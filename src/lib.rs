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
/// let cond = join_and([User::Name.eq("John"), User::Age.gt(30)]);
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
/// let cond = join_or([User::Name.eq("John"), User::Name.eq("Jack")]);
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

/// produces `COALESCE({exprs})`
///
/// # Example
///
/// ```no_run
/// let expr = coalesce([User::FirstName, User::LastName])
/// assert_eq!(expr, r#"COALESCE("User"."first_name", "User"."last_name")"#)
/// ```
pub fn coalesce<I>(exprs: I) -> String
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let exprs = exprs
        .into_iter()
        .map(|c| c.into())
        .collect::<Vec<_>>()
        .join(", ");
    format!("COALESCE({exprs})")
}

/// produces `COALESCE({exprs}) AS {alias}`
///
/// # Example
///
/// ```no_run
/// let expr = coalesce("username", [User::FirstName, User::LastName])
/// assert_eq!(expr, r#"COALESCE("User"."first_name", "User"."last_name") AS "username""#)
/// ```
pub fn coalesce_as<I>(alias: &str, exprs: I) -> String
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    format!("{} AS {alias:?}", coalesce(exprs))
}
