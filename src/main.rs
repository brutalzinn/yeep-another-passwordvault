use api::storage::storage::init;
use clap::Parser;
mod api;
#[derive(Parser)] // requires `derive` feature
#[command(name = "yeep")]
#[command(bin_name = "yeep")]
enum Cli {
    Add(AddArgs),
    Del(DelArgs),
    Save(SaveArgs)
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = "Save all")]
struct SaveArgs {
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = "Add new secret keypair")]
struct AddArgs {
    #[arg(long)]
    key: Option<String>,

    #[arg(long)]
    value: Option<String>,
}
#[derive(clap::Args)]
#[command(author, version, about, long_about = "Remove a secret")]
struct DelArgs {
    #[arg(long)]
    key: Option<String>,
}

fn main() {
    let result     =  init();
    let args = Cli::parse();
    match args {
        Cli::Add(args) => {
            println!("Add {:?}, {:?}", args.key, args.value);
        }
        Cli::Del(args) => {
            println!("Del {:?}", args.key);
        }
        Cli::Save(_) => {
            println!("Saving..");
        }
    }
}