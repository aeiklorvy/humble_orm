use humble_orm::*;

generate_structs_sqlite! {
    CREATE TABLE UserData (
        id INTEGER PRIMARY KEY,
        userValue1 varchar(256) NOT NULL,
        userValue2 INTEGER NOT NULL
    );
    CREATE TABLE User (
        id INTEGER PRIMARY KEY,
        name varchar(64) NOT NULL,
        data_id INTEGER DEFAULT NULL,
        FOREIGN KEY (data_id) REFERENCES UserData (id)
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // create sqlite database in memory
    let url = "sqlite::memory:";
    let pool = sqlx::SqlitePool::connect(url).await.unwrap();

    // create table UserData
    UserData::create_table(&pool).await.unwrap();
    // create table User
    User::create_table(&pool).await.unwrap();

    // Add a record to the UserData so that it can be referenced later
    {
        let data = UserData {
            id: 1,
            user_value_1: "some user value".into(),
            user_value_2: 12345,
        };
        data.insert(&pool).await.unwrap();
        // make sure that the record is added
        assert_eq!(UserData::get_by_id(&pool, 1).await.unwrap().id, data.id);
    }

    // Now let's add a User
    {
        let mut user = User {
            id: 0, // will be replaced by the DBMS value
            data_id: Some(1),
            name: "Bob".into(),
        };
        user.insert_generating_id(&pool).await.unwrap();
        // make sure that id was updated
        assert_eq!(user.id, 1);

        // make sure that the record is added
        let user2 = User::get_by_id(&pool, 1).await.unwrap();
        assert_eq!(user2.id, user.id);
        assert_eq!(user2.name, user.name);
    }

    // I want more users!
    User {
        id: 2,
        name: "Jack".into(),
        data_id: Some(1),
    }
    .insert(&pool)
    .await
    .unwrap();

    // But this user will be unrelated to another table
    User {
        id: 3,
        name: "John".into(),
        data_id: None, // means NULL
    }
    .insert(&pool)
    .await
    .unwrap();

    // Let's enjoy the result! Should print this:
    //
    // User { id: 1, name: "Bob", data_id: Some(1) }
    // UserData { id: 1, user_value_1: "some user value", user_value_2: 12345 }
    //
    // User { id: 2, name: "Jack", data_id: Some(1) }
    // UserData { id: 1, user_value_1: "some user value", user_value_2: 12345 }
    //
    // User { id: 3, name: "John", data_id: None }
    // no user data
    //
    for user in User::select_all(&pool).await.unwrap() {
        println!("{user:?}");
        if let Some(id) = user.data_id {
            let user_data = UserData::get_by_id(&pool, id).await.unwrap();
            println!("{user_data:?}");
        } else {
            println!("no user data");
        }
        println!(); // line break
    }
}
