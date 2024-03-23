use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use shellexpand::tilde;

const DATABASE_PATH: &str = "~/.config/thoth/db.sqlite";

async fn run() -> Result<(), DbErr> {
    let _ =
        Database::connect(format!("sqlite:{}?mode=rwc", tilde(DATABASE_PATH)))
            .await?;

    Ok(())
}

pub fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
