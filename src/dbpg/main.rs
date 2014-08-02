extern crate postgres;
extern crate time;

use std::os;
use time::Timespec;
use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;

struct Row {
    id: i32,
    key: String,
    value: String,
    last_modified: Timespec,
    data: Option<Vec<u8>>,
}

fn create_tables(conn: &PostgresConnection) {
    conn.execute("CREATE TABLE IF NOT EXISTS row (
                    id SERIAL PRIMARY KEY)", []).unwrap();
}

fn insert_row(conn: &PostgresConnection, row: &Row) /*-> Row*/ {
    conn.execute(
        "INSERT INTO row (key, value, last_modified, data) VALUES ($1, $2, $3, $4)",
        [&row.key, &row.value, &row.last_modified, &row.data]).unwrap();

    // re-retrieve it so we can get to new id
}

fn main() {
    let conn = PostgresConnection::connect("postgres://dbpg_user@localhost/dbpg_db", &NoSsl).unwrap();
    create_tables(&conn);

    let hd = os::homedir().unwrap();
    let hd2 = hd.display();

    println!("{}", hd2);

    let mut r1 = Row {
        id: 0,
        key: "TestKey".to_string(),
        value: "TestValue".to_string(),
        last_modified: time::get_time(),
        data: None
    };

    let mut r2 = Row {
        id: 0,
        key: "TestKey".to_string(),
        value: "TestValue".to_string(),
        last_modified: time::get_time(),
        data: Some(vec![0u8, 1u8, 2u8]),
    };

    insert_row(&conn, &mut r1);
    insert_row(&conn, &mut r2);
}
