mod api;
use std::{string, env};

use api::{config::config::read_file, cli::cli::Cli, storage::storage::read_keypairs};
use rusqlite::Connection;
use clap::Parser;
/// this is the moment that i understand the practice of use mod.rs or a crate to inform some especification to compiler. But dude, this is not a human readable at all..

use crate::api::{storage::storage::{insert_keypair, read_keypair, delete_keypair}, crypt::crypt::{crypt, decrypt}};

fn main() {
    let filename= "config.toml";
    let data = read_file(filename);
    const VERSION: &str = env!("CARGO_PKG_VERSION");
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
        Cli::Version(_) => {
            println!("Current version is: {}", VERSION)
        }
        Cli::Setup(_) => {

            let current_dir: String = match  env::current_dir(){
                Ok(result)=> result.to_string_lossy().to_string(),
                Err(_) => "cant set current dir".to_string()
            };
            
            #[cfg(target_os = "linux")]
            {
                println!("For Linux or macOS (Bash):");
                println!("   1. Open your Bash profile file with a text editor (e.g., vim or nano):");
                println!("      - Example: vim ~/.bashrc");
                println!("   2. Add the following line at the end of the file:");
                println!("      - Example: export PATH=$PATH:{}",current_dir);
                println!("   3. Save and exit the text editor.");
            }
        
            #[cfg(target_os = "macos")]
            {
                println!("For macOS (Bash):");
                println!("   1. Open your Bash profile file with a text editor (e.g., vim or nano):");
                println!("      - Example: vim ~/.bash_profile");
                println!("   2. Add the following line at the end of the file:");
                println!("      - Example: export PATH=$PATH:{}",current_dir);
                println!("   3. Save and exit the text editor.");
            }
        
            #[cfg(target_os = "windows")]
            {
                println!("For Windows:");
                println!("   1. Open the Start menu and search for 'Environment Variables'.");
                println!("   2. Click 'Edit the system environment variables'.");
                println!("   3. Click the 'Environment Variables...' button.");
                println!("   4. In the 'System variables' section, select the 'Path' variable and click 'Edit'.");
                println!("   5. Click 'New' and add the path to your 'bin' directory (e.g., {}).", current_dir);
                println!("   6. Click 'OK' to close each dialog.");
            }
        
        }
    }
}