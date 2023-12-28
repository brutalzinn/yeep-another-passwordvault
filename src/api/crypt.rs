///agree with https://github.com/magiclen/rust-magiccrypt now. Ok. i suppose to do simple things in rust.. its not soo madure at this time. This is because i still compare some other tools with Rust XD

pub mod crypt{
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};

    pub fn crypt(value: &str, secret: &str) -> String{
        let mcrypt = new_magic_crypt!(secret, 256); //Creates an instance of the magic crypt library/crate.
        let encrypted_string = mcrypt.encrypt_str_to_base64(value); //Encrypts the string and saves it to the 'encrypted_string' variable.
        return encrypted_string;
    }

    pub fn decrypt(encrypted: &str, secret: &str) -> String{
        let mcrypt = new_magic_crypt!(secret, 256); //Creates an instance of the magic crypt library/crate.
        let decrypted_string = mcrypt.decrypt_base64_to_string(&encrypted).unwrap(); //Decrypts the string so we can read it.
        println!("Decrypted String: {}", decrypted_string); //Print the human-readable, decrypted string.
        return decrypted_string;
    }
}