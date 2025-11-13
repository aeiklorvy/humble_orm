# Humble ORM

This is a small add-on to sqlx that provides some ORM capabilities for a
better DBMS experience

## Installation

Required crate `sqlx` with the `time` feature:
```toml
sqlx = { version = "*", features = ["time"] }
```
This is the most necessary, you can add the rest to your taste.

You can also specify the `serde` feature so that the generated structures
support serialization and deserialization. If you want to use `serde`, then
you will need to add a dependency to the project:

```toml
serde = { version = "*", features = ["derive"] }
```

## Models/Entities generation

Let's imagine that we have a database schema, and we can wrap it in a macro,
getting structures in the Rust language!

By writing such a macros:
```sql
generate_structs_mysql! {
    CREATE TABLE order (
        id INT NOT NULL AUTO_INCREMENT,
        create_date DATE,
        PRIMARY KEY (id)
    );

    CREATE TABLE order_details (
        id INT NOT NULL AUTO_INCREMENT,
        order_id INT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (order_id) REFERENCES order (id) ON DELETE CASCADE
    );
}
```
The following code will be generated under the hood:
```Rust
pub struct Order {
    id: i64,
    create_date: Option<sqlx::time::Date>,
}
pub struct OrderDetails {
    id: i64,
    order_id: i64,
}
```

In the examples below, I will rely on these structures, remember them.

Note that the suffix with the DBMS type in the macro name is very important –
it sets some syntax settings for the SQL parser and affects the type names in
the generated code. Choose the appropriate macro name according to the sqlx
features in your project.

## ORM capabilities

Each generated structure will have the following methods:
- `select_all`: selects all table rows from the database;
- `insert`: inserts a record by sending all columns of the model;
- `create_table`: сreates a new table in the database;

If a primary key has been defined in the table:
- `get_by_primary_key`: selects a record by primary key value;
- `update`: updates the record where primary key value matches the one in
  model;
- `delete`: deletes the record where primary key value matches the one in
  model;
- `insert_generating_primary_key`: same as `insert`, but gets primary key
  value from the DBMS;

Also, each structure, in addition to the fields containing the value, will
contain a constant representing the entity corresponding to the column name.
This is nothing more than information for building queries. For example, the
struct `OrderDetails` will have `Id` and `OrderId`, and `Order` will have
`ID` and `CreateDate`. See the `Query Building` section for more details.

So, let's summarize and figure out what methods and constants the
`OrderDetails` struct will have:
- `Id`, `OrderId`: constants that contains info about columns (for query building);
- `select_all`, `insert`: basic orm methods;
- `get_by_primary_key`, `insert_generating_primary_key`, `update`, `delete`:
  because `PRIMARY KEY` was defined;

## What generates the macro

The generated structs `Order` and `OrderDetails` will also be complemented by
the impl:

```Rust
/// Selects a row from the database where the primary key corresponds to the
/// specified value
///
/// Internally, it uses prepared statements, so this is the most preferred way
/// to get a single row of the table using the primary key
async fn get_by_#[primary_key](pool: &#[pool], value: #[primary_key_type]) -> Result<Self, sqlx::Error>;

/// Selects all table rows from the database
async fn select_all(pool: &#[pool]) -> Result<Vec<Self>, sqlx::Error>;

/// Updates a row in the database that corresponds to the value of the primary
/// key field
async fn update(&self, pool: &#[pool]) -> Result<(), sqlx::Error>;

/// Inserts a record via `INSERT` by sending all columns of the model
async fn insert(&self, pool: &#[pool]) -> Result<(), sqlx::Error>;

/// Inserts a record via `INSERT`, skipping the primary key field, and after
/// insertion sets the primary key value from the DBMS to the model
async fn insert_generating_#[primary_key](&mut self, pool: &#[pool]) -> Result<(), sqlx::Error>;

/// Deletes a row in the database that corresponds to the value of the primary
/// key field, and the model will be consumed
async fn delete(self, pool: &#[pool]) -> Result<(), sqlx::Error>;

/// Creates a new table in the database
async fn create_table(pool: &#[pool], drop_if_exists: bool) -> Result<(), sqlx::Error>;

/// Deletes the entire table from the database, does nothing if there is no
/// such table
async fn drop_table(pool: &#[pool]) -> Result<(), sqlx::Error>;
```

Separately to the structure, there will also be a module with type aliases for
the fields of the structure:

```Rust
pub mod OrderColumnTypes {
    pub type IdType = i64;
    pub type CreateDateType = Option<sqlx::time::Date>;
}
pub mod OrderDetailsColumnTypes {
    pub type IdType = i64;
    pub type OrderIdType = i64;
}
```

## Using сolumn types

The user is free to choose how to read the query, and therefore it is
necessary to know the data type for the table column in order to extract the
value correctly.

Along with the struct, the module `name + "ColumnTypes"` will also be
generated by macro, containing type aliases for each column:
`column + "Type"`. These type aliases exist to help match types in code and
prevent data extraction errors. For example, take this method of data
extraction:

```Rust
let sql = "SELECT id AS value FROM order";
for row in sqlx::query(sql).fetch_all(pool).await.unwrap() {
    let value: i64 = row.get("value");
}
```

Here, `value` is always extracted with the specified type. However, as soon as
the table schema or query changes, these fields may be of a different type!
For example, if `value` suddenly becomes `String`, then we will constantly
encounter an extraction error. By using type aliases to extract values, these
errors can be avoided in advance – the code simply does not compile!

```Rust
let sql = "SELECT id AS value FROM order";
for row in sqlx::query(sql).fetch_all(pool).await.unwrap() {
    // it compiles as long as the `IdType` is `i64`
    let value: i64 = row.get::<OrderColumnTypes::IdType, _>("value");
}
```

## Query building

The library also has the ability to generate queries of any complexity.
However, so far this is limited to `SELECT` only, because the library is
under active development :)

The library does not execute queries, but only generates a raw string that the
DBMS must execute (for example, via `sqlx::query`). The user is free to
choose how to send the request and how to read its result.

Take a look at an example of `SELECT` that can be approximated to real-world
tasks:

```Rust
// introduce some filters
let start_date = time::date!(2025 - 01 - 01);
let end_date = time::date!(2025 - 12 - 31);

// SELECT "order"."create_date"
//        "order_detail".*
// FROM "order"
// LEFT JOIN "order_detail" ON "order"."id" = "order_detail"."order_id"
// WHERE "order"."create_date" BETWEEN "2025-01-01" AND "2025-12-31"
let sql: String = Select::new()
    .with_column(Order::CreateDate)
    .with_column(OrderDetail::All)
    .with_table(Order::as_table())
    .with_left_join(OrderDetail::as_table(), [Order::Id.eq(OrderDetail::OrderId)])
    .with_where_cond(Order::CreateDate.between(start_date, end_date))
    .build();
```

Of course, this query can be rewritten in a different style to look a little
more uncomplicated with heavy construction logic:

```Rust
let mut select = Select::new();

select.push_column(Order::CreateDate);
select.push_column(OrderDetail::All);

select.set_table(Order::as_table());
select.left_join(OrderDetail::as_table(), [Order::Id.eq(OrderDetail::OrderId)]);

select.push_where_cond(Order::CreateDate.between(start_date, end_date));

// the result will be the same
let sql: String = select.build();
```
