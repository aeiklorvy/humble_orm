# What is this

This is a small add-on to sqlx that provides some ORM capabilities for a better DBMS experience

## Installation

Required crate `sqlx` with the `time` feature:
```toml
sqlx = { version = "*", features = ["time"] }
```
This is the most necessary, you can add the rest to your taste.

## Models/Entities generation

Let's imagine that we have a database schema, and we can wrap it in a macro, getting structures in the Rust language!

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

Note that the suffix with the DBMS type in the macro name is very important – it sets some syntax settings for the SQL parser and affects the type names in the generated code. Choose the appropriate macro name according to the sqlx features in your project.

## ORM capabilities

Each generated structure will have the following methods:
- `select_all`: selects all table rows from the database;
- `insert`: inserts a record by sending all columns of the model;

If a primary key has been defined in the table:
- `get_by_primary_key`: selects a record by primary key value;
- `update`: updates the record where primary key value matches the one in model;
- `delete`: deletes the record where primary key value matches the one in model;
- `insert_generating_primary_key`: same as `insert`, but gets primary key value from the DBMS;

If a foreign key has been defined in the table:
- `get_related_by_` + name: selects a row of the related table according to field value in the model;
- `get_all_related_by_` + name: selects all rows of the related table according to field value in the model;

Also, each structure, in addition to the fields containing the value, will contain a constant representing the entity corresponding to the column name.
This is nothing more than information for building queries.
For example, the struct `OrderDetails` will have `ID` and `ORDER_ID`, and `Order` will have `ID` and `CREATE_DATE`.
See the `Query Building` section for more details.

To get values from `sqlx::Row` library will generates methods with the following name:
- field name + `_from_row`;
- field name + `_try_from_row`;

They rely internally on `sqlx::Row::get` and don't do anything out of the ordinary.
Their purpose is to help match types in code and prevent mistakes when extracting the wrong type.
For example, if the user gets the values like this:
```Rust
for row in sqlx::query("...").fetch_all(pool).await.unwrap() {
    let name: String = row.get("name");
    let value: i64 = row.get("value");
}
```
And if the database schema has changed later, then this code will still try to extract values with the old types, and this will inevitably lead to a panic.
Using the generated methods, this can be avoided in advance – the code simply does not compile!
```Rust
for row in sqlx::query("...").fetch_all(pool).await.unwrap() {
    // actually, there is no need for type annotations
    let name: String = SomeTable::name_from_row(&row);
    let value: i64 = SomeTable::value_from_row(&row);
}
```

So, let's summarize and figure out what methods and constants the `OrderDetails` struct will have:
- `ID`, `ORDER_ID`: constants that contains info about columns (for query building);
- `select_all`, `insert`: basic orm methods;
- `get_by_primary_key`, `insert_generating_primary_key`, `update`, `delete`: because `PRIMARY KEY` was defined;
- `get_related_by_order_id`, `get_all_related_by_order_id`: bescause `FOREIGN KEY` was defined;
- `id_from_row`, `id_try_from_row`: methods that gets `id` value from `sqlx::Row` with corresponding type (`i64`);
- `order_id_from_row`, `order_id_try_from_row`: methods that gets `order_id` value from `sqlx::Row` with corresponding type (`i64`);

## Query building

The library also has the ability to generate queries of any complexity. However, so far this is limited to `SELECT` only, because the library is under active development :)

The library does not execute queries, but only generates a raw string that the DBMS must execute (for example, via `sqlx::query`). The user is free to choose how to send the request and how to read its result.

Take a look at an example of `SELECT` that can be approximated to real-world tasks:
```Rust
// introduce some filters
let start_date = time::date!(2025 - 01 - 01);
let end_date = time::date!(2026 - 12 - 31);

// SELECT "order"."create_date"
//        "order_detail".*
// FROM "order"
// LEFT JOIN "order_detail" ON "order"."id" = "order_detail"."order_id"
// WHERE "order"."create_date" BETWEEN "2025-01-01" AND "2025-12-31"
let sql: String = Select::new()
    .with_column(Order::CREATE_DATE)
    .with_column(OrderDetail::ALL)
    .with_table(Order::as_table())
    .with_left_join(OrderDetail::as_table(), [Order::ID.eq(OrderDetail::ORDER_ID)])
    .with_where_cond(Order::CREATE_DATE.between(start_date, end_date))
    .build();
```
Of course, this query can be rewritten in a different style to look a little more uncomplicated with heavy construction logic:
```Rust
let mut select = Select::new();

select.push_column(Order::CREATE_DATE);
select.push_column(OrderDetail::ALL);

select.set_table(Order::as_table());
select.left_join(OrderDetail::as_table(), [Order::ID.eq(OrderDetail::ORDER_ID)]);

select.push_where_cond(Order::CREATE_DATE.between(start_date, end_date));

// the result will be the same 
let sql: String = select.build();
```
