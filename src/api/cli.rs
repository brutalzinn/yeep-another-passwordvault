pub mod cli {
    extern crate clap;
    use clap::Parser;

    #[derive(Parser)] 
    #[command(name = "yeep")]
    #[command(bin_name = "yeep")]
    pub enum Cli {
        List(ListArgs),
        Add(AddArgs),
        Read(ReadArgs),
        Del(DelArgs),
        Babayaga(BabayaArgs)
    }
    
    #[derive(clap::Args)]
    #[command(author, version, about, long_about = "Read a keypair")]
    pub struct ReadArgs {
        #[arg(long)]
        pub id: i32,
    
        #[arg(long)]
        pub secret: Option<String>
    }
    
    #[derive(clap::Args)]
    #[command(author, version, about, long_about = "babayaga. maybe.. a JOHN WICK easter egg?!")]
    pub struct BabayaArgs {
        #[arg(long)]
        pub agent: Option<String>
    }
    
    #[derive(clap::Args)]
    #[command(author, version, about, long_about = "List all")]
    pub struct ListArgs {
    }
    
    #[derive(clap::Args)]
    #[command(author, version, about, long_about = "Add new secret keypair")]
    pub struct AddArgs {
        #[arg(long)]
        pub key: String,
    
        #[arg(long)]
        pub value: String,
    
        #[arg(long)]
        pub secret: Option<String>
    }
    #[derive(clap::Args)]
    #[command(author, version, about, long_about = "Delete a keypair")]
    pub struct DelArgs {
        #[arg(long)]
        pub id: i32,
    }
}