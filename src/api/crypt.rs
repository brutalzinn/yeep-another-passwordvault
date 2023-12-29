///agree with https://github.com/magiclen/rust-magiccrypt now. Ok. i suppose to do simple things in rust.. its not soo madure at this time. This is because i still compare some other tools with Rust XD

pub mod crypt{
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};

    pub fn crypt(value: &str, secret: &str) -> String{
        let mcrypt = new_magic_crypt!(secret, 256); 
        let encrypted_string = mcrypt.encrypt_str_to_base64(value); 
        return encrypted_string;
    }

    pub fn decrypt(encrypted: &str, secret: &str) -> String{
        let mcrypt = new_magic_crypt!(secret, 256); 
        let decrypted_string = mcrypt.decrypt_base64_to_string(&encrypted);
        let result = match decrypted_string {
            Ok(data)=> data,
            Err(err)=> panic!("BABAYAHA IS HERE.. OR Maybe wrong secret key?! {}",err) 
        };
        return result;
    }
}