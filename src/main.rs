mod schema;
mod twitter;

use console::Term;
use dialoguer::{Input, Password};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;
use std::io::Result;

type DbPool = Pool<ConnectionManager<PgConnection>>;

// TODO: Return a Result instead of panicking
fn establish_connection_pool(term: &Term, db_url: &str) -> DbPool {
    term.write_line("[+] Connecting to database...").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Pool::builder()
        .build(manager)
        .expect("Postgres connection pool could not be created")
}

fn print_header(term: &Term) -> Result<()> {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let subtitle = format!("v{pkg_version} by @pascopatcho");
    let right_aligned_subtitle = format!("{subtitle:>52}");
    term.write_line("")?;
    term.write_line("░▒█▀▀▀░░▀░░█▀▀▄░█▀▀░█░░░█░█▀▀▄░▀█▀░█▀▄░█░░░░█▀▀░█▀▀▄")?;
    term.write_line("░▒█▀▀░░░█▀░█▄▄▀░█▀▀░▀▄█▄▀░█▄▄█░░█░░█░░░█▀▀█░█▀▀░█▄▄▀")?;
    term.write_line("░▒█░░░░▀▀▀░▀░▀▀░▀▀▀░░▀░▀░░▀░░▀░░▀░░▀▀▀░▀░░▀░▀▀▀░▀░▀▀")?;
    term.write_line(&right_aligned_subtitle)?;
    term.write_line("")?;

    Ok(())
}

fn database_configuration_wizard(term: &Term) -> Result<String> {
    term.write_line("[+] Starting database configuration wizard.")?;
    term.write_line("")?;

    let hostname: String = Input::new()
        .with_prompt("Enter PostgreSQL hostname")
        .default("localhost".into())
        .interact_on(term)?;

    let port: String = Input::new()
        .with_prompt("Enter PostgreSQL port")
        .default("5432".into())
        .interact_on(term)?;

    let username: String = Input::new()
        .with_prompt("Enter PostgreSQL username")
        .default("firewatcher".into())
        .interact_on(term)?;

    let password: String = Password::new()
        .with_prompt("Enter PosgreSQL password")
        .interact_on(term)?;

    let database_name: String = Input::new()
        .with_prompt("Enter PostgreSQL database name")
        .default("firewatcher".into())
        .interact_on(term)?;

    //TODO: Save the database url in the .env file for later use

    Ok(format!(
        "postgres://{username}:{password}@{hostname}:{port}/{database_name}"
    ))
}

fn twitter_credentials_wizard(term: &Term, db_pool: &DbPool) -> Result<()> {
    term.write_line("[+] Starting twitter credentials wizard.")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().unwrap(); // TODO: Panick more nicely
    let term = Term::stdout();
    print_header(&term)?;

    let database_url = env::var("DATABASE_URL").or_else(|_| {
        term.write_line("[+] Database configuration not found.")?;
        database_configuration_wizard(&term)
    })?;

    let db_pool = establish_connection_pool(&term, &database_url);

    let twitter_credentials_count = {
        let credentials_manager = twitter::CredentialsManager::new(&db_pool);
        credentials_manager.count().unwrap() // TODO: panick more nicely
    };

    if twitter_credentials_count <= 0 {
        term.write_line("[+] No twitter credentials found.")?;
        twitter_credentials_wizard(&term, &db_pool)?;
    }

    Ok(())
}
