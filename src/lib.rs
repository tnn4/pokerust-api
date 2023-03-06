//! rusqlite1 lib.rs
//! 

pub mod pokerust;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use std::collections::HashMap;

// Some cat examples from the rusqlite website
#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

// Turn these into tests
pub fn create_cat_db() -> Result<()> {
    // Connection open will create the database if it does not exist
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "CREATE TABLE if not exists cat_colors (
            id integer primary key,
            name text not null unique
        )",
        NO_PARAMS,
    )?;
    conn.execute(
        "CREATE TABLE if not exists cats(
            id integer primary key,
            name text not null,
            color_id integer not null references cat_colors(id)
        )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn insert_and_select_cats() -> Result <()> {
    let conn = Connection::open("cats.db")?;

    // Data to insert
    // color, list of cats
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Green"), vec!["Sprigatito", "Floragato", "Meowscarada"]);
    cat_colors.insert(String::from("Gray"), vec!["Glameow", "Purugly"]);
    cat_colors.insert(String::from("Cream"), vec!["Meowth", "Persian"]);
    cat_colors.insert(String::from("Pink"), vec!["Skitty", "Delcatty"]);

    // Insert data into table
    for (color, catnames) in &cat_colors {
        // cat colors
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;

        let last_id: String = conn.last_insert_rowid().to_string();
        // iterate through the vector
        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }

    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats AS c
         INNER JOIN cat_colors AS cc
         ON cc.id = c.color_id;"
    )?;

    let cats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())

}