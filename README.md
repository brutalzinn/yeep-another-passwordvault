# My first CLI (command line interface) with Rust

 A small repo to give me the feelings of Rust development ( maybe i use it to Arduino development ??? YES ). 
 
 The objective of this project is create a CLI tool to store passwords or other secret data using a pass secrey key in agree with AES(Advanced Encryption Standard) 

 # OPTION 1 - Getting Started and compile the yeep

1. Clone this repo using `$ git clone https://github.com/brutalzinn/yeep-another-passwordvault.git`
2. Install rust
3. Open you command prompt and execute the next commands `$ cargo update` and `$ cargo build` 
4. go to folder target/debug
5. Add you first keypair data `$ run yeep add --key myawesomeky --value myhighvaluesensitivedata`
6. Retrive you list of keypairs `$ run yeep list`
7. Retrive you first crypted data  `$ run yeep read --id 0`
8. Enjoy. You can use more commands. `$ run yeep help` to get more.

# OPTION 2 - Use a ready to use release

1. check this repo release page [releases](https://github.com/brutalzinn/yeep-another-passwordvault/releases)
2. Choose you system version 
3. Download zip file
4. Extract the zip folder
5. Open your prompt command at the extracted directory.
6. Add you first keypair data `$ run yeep add --key myawesomeky --value myhighvaluesensitivedata`
7. Retrive you list of keypairs `$ run yeep list`
8. Retrive you first crypted data  `$ run yeep read --id 0`
9. Enjoy. You can use more commands. `$ run yeep help` to get more.


 # Usage

      Usage: yeep <COMMAND>

      Commands:
      list      List all
      add       Add new secret keypair
      read      Read a keypair
      del       Delete a keypair
      babayaga  babayaga. maybe.. a JOHN WICK easter egg?!
      setup     Show setup instructions to configure yeep as global CLI
      version   get current version
      help      Print this message or the help of the given subcommand(s)

      Options:
      -h, --help  Print help