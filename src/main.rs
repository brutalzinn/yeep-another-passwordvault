use crate::api::storage::storage::{create_table, insert_keypair, read_keypairs};
use clap::Parser;
use rusqlite::Connection;
mod api;
#[derive(Parser)] 
#[command(name = "yeep")]
#[command(bin_name = "yeep")]
enum Cli {
    List(ListArgs),
    Add(AddArgs),
    Del(DelArgs),
    Save(SaveArgs)
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = "Save all")]
struct SaveArgs {
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = "List all")]
struct ListArgs {
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = "Add new secret keypair")]
struct AddArgs {
    #[arg(long)]
    key: String,

    #[arg(long)]
    value: String,
}
#[derive(clap::Args)]
#[command(author, version, about, long_about = "Remove a secret")]
struct DelArgs {
    #[arg(long)]
    id: Option<i32>,
}

fn main() {
    let conn = Connection::open("test.db").unwrap();
    create_table(&conn).unwrap();
    let args = Cli::parse();
    match args {
        Cli::Add(args) => {
            println!("Add {:?}, {:?}", args.key, args.value);
            insert_keypair(&conn, &args.key , &args.value).unwrap();   //.unwrap();
        }
        Cli::Del(args) => {
            println!("Del {:?}", args.id);
        }
        Cli::Save(_) => {
            println!("Saving..");
        }
        Cli::List(_) => {
            let keypairs = read_keypairs(&conn).unwrap();
            for item in keypairs {
                println!("{} {}", item.id, item.key);
            }
        }
    }
}