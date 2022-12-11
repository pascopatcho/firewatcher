mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::io::{stdout, Write};
use console::Term;
use dialoguer::Input;


type PgPool = Pool<ConnectionManager<PgConnection>>;

fn establish_connection_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Postgres connection pool could not be created")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = establish_connection_pool(&database_url);
    let term = Term::stdout();



    term.write_line("Je teste un autre truc !")?;
    let input: String = Input::new()
        .with_prompt("Comment on fait une révolution")
        .with_initial_text("GREVE")
        .interact()?;
    term.write_line(&input);



    Ok(())
}