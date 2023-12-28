use crate::api::{storage::storage::{create_table, insert_keypair, read_keypairs}, crypt::{crypt::{crypt, decrypt}}};
use api::{config::{self, config::read}, storage::storage::read_keypair};
use clap::Parser;
use rusqlite::Connection;
mod api;
#[derive(Parser)] 
#[command(name = "yeep")]
#[command(bin_name = "yeep")]
enum Cli {
    List(ListArgs),
    Add(AddArgs),
    Read(ReadArgs),
    Del(DelArgs),
    Save(SaveArgs)
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = "Read a keypair")]
struct ReadArgs {
    #[arg(long)]
    id: i32,

    #[arg(long)]
    secret: Option<String>
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

    #[arg(long)]
    secret: Option<String>
}
#[derive(clap::Args)]
#[command(author, version, about, long_about = "Remove a secret")]
struct DelArgs {
    #[arg(long)]
    id: Option<i32>,
}

fn main() {
    let data = read("config.toml");
    let conn = Connection::open("test.db").unwrap();
    create_table(&conn).unwrap();
    let args = Cli::parse();
    match args {
        Cli::Add(args) => {
            println!("Add {:?}, {:?}", args.key, args.value);

            let secret = match args.secret{
                Some(option)=> option,
                None=> data.config.secret
            } ;
            println!("Secret key: {}", secret);
            let cryptData = crypt(&args.value, secret.as_str());
            insert_keypair(&conn, &args.key , &cryptData).unwrap();   //.unwrap();
        }
        Cli::Read(args)=> {
            let keypairs = read_keypair(&conn,args.id).unwrap();
            let result = match keypairs {
                Some(option)=> option,
                None=> panic!("no found")
            };
            let secret = match args.secret{
                Some(option)=> option,
                None=> data.config.secret
            } ;
            let readData = decrypt(&result.value, &secret);

            println!("{} {}", result.id, readData);

        },
        Cli::Del(args) => {
            println!("Del {:?}", args.id);
        }
        Cli::Save(_) => {
            println!("Saving..");
        }
        Cli::List(_) => {
            let keypairs = read_keypairs(&conn).unwrap();
            for item in keypairs {
                println!("{} {}", item.id, item.key)
            }
        }
    }
}