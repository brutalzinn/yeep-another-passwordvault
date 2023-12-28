pub mod config {
    use serde_derive::Deserialize;
    use std::fs;
    use toml;

    #[derive(Deserialize)]
    pub struct Data {
        pub config: Config
    }

    #[derive(Deserialize)]
    pub struct Config {
        pub secret: String
    }


 pub fn read(filename: &str) -> Data {

        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                panic!("Could not read file `{}`", filename);
            }
        };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            panic!("Unable to load data from `{}`", filename);
        }
    };
    return data
    }
}