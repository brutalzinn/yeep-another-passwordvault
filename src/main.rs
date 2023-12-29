mod api;
use api::{config::config::read_file, cli::cli::Cli, storage::storage::read_keypairs};
use rusqlite::Connection;
use clap::Parser;
/// this is the moment that i understand the practice of use mod.rs or a crate to inform some especification to compiler. But dude, this is not a human readable at all..

use crate::api::{storage::storage::{insert_keypair, read_keypair, delete_keypair}, crypt::crypt::{crypt, decrypt}};

fn main() {
    let filename= "config.toml";
    let data = read_file(filename);

    let conn = Connection::open("test.db").unwrap();
    api::storage::storage::create_table(&conn).unwrap();
    let args = Cli::parse();
    match args {
        Cli::Add(args) => {
            println!("Add {:?}, {:?}", args.key, args.value);
            let secret = match args.secret{
                Some(option)=> option,
                None=> data.config.secret
            } ;
            println!("Secret key: {}", secret);
            let crypt_data = crypt(&args.value, secret.as_str());
            insert_keypair(&conn, &args.key , &crypt_data).unwrap();   //.unwrap();
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
            let decrypt_data = decrypt(&result.value, &secret);
            println!("{} {}", result.id, decrypt_data);

        },
        Cli::Del(args) => {
            let result = delete_keypair(&conn, args.id);
            match result {
            Ok(_)=>    println!("Deleted {:?} sucefull", args.id),
                Err(_)=> panic!("Failed to delete the key {}", args.id) 
            };
        }
        Cli::Babayaga(args) => {
            let target = match args.agent{
                Some(option)=> option,
                None=> "YOU. I think you cant do nothing..".to_string()
            } ;
            println!("BABAYAGA IS COMING TO kill {}", target);
        }
        Cli::List(_) => {
            let keypairs = read_keypairs(&conn).unwrap();
            for item in keypairs {
                println!("{} {}", item.id, item.key)
            }
        }
    }
}