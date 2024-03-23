use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Score {
    title: String,
    composer: String,
}

pub fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;

    connection.execute(
        "CREATE TABLE score (
            title TEXT NOT NULL,
            composer TEXT NOT NULL
        )",
        (),
    )?;

    let score = Score {
        title: "Piano Piece".to_string(),
        composer: "Ben Rosen".to_string(),
    };

    connection.execute(
        "INSERT INTO score (title, composer) VALUES (?1, ?2)",
        (&score.title, &score.composer),
    )?;

    let mut statement =
        connection.prepare("SELECT title, composer FROM score")?;

    let scores = statement.query_map([], |row| {
        Ok(Score {
            title: row.get(0)?,
            composer: row.get(1)?,
        })
    })?;

    for score in scores {
        match score {
            Ok(score) => println!("Found score {:?}", score),
            Err(message) => println!("{message}"),
        }
    }

    Ok(())
}
