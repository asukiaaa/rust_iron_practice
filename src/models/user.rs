use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use time::Timespec;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub time_created: Timespec,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("time_created", "time")?; // TODO: parse &self.time_created;
        state.end()
    }
}

impl User {
    fn get_conn() -> Connection {
        Connection::open("data.db").unwrap()
    }

    pub fn init_table() {
        let conn = User::get_conn();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  email           TEXT NOT NULL UNIQUE,
                  time_created    TEXT NOT NULL
                  )",
            NO_PARAMS,
        ).unwrap();
        let me = User {
            id: 0,
            name: "Steven".to_string(),
            email: "steven@example.com".to_string(),
            time_created: time::get_time(),
        };
        me.save();
    }

    fn save(&self) {
        let action = if self.id == 0 { "INSERT" } else { "REPLACE" };
        let query = action.to_string() + " "
            + "INTO users (name, email, time_created)
               VALUES (?1, ?2, ?3)";
        let result = User::get_conn().execute(
            query.as_str(),
            &[&self.name as &ToSql, &self.email as &ToSql, &self.time_created],
        );
        println!("save result: {:?}", result);
        // TODO error handling
    }

    pub fn find(id: i32) -> Option<User> {
        let conn = User::get_conn();
        let users = conn.query_row(
            "SELECT * FROM users WHERE id = ?1",
            &[&id],
            |row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                time_created: row.get(3),
            }
        );
        println!("users: {:?}", users);
        for u in users {
            println!("user: {:?}", u);
            return Some(u);
        }
        None
    }

    pub fn find_all() -> Vec<User> {
        let conn = User::get_conn();
        let users_iter = conn.query_row(
            "SELECT * FROM users",
            NO_PARAMS,
            |row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                time_created: row.get(3),
            }
        );
        let mut users = Vec::new();
        for user in users_iter {
            users.push(user);
        }
        users
    }
}
