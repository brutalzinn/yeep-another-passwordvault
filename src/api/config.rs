pub mod config {
    use serde_derive::{Deserialize, Serialize};
    use std::fs;
    use toml;

    #[derive(Deserialize)]
    #[derive(Serialize)]
    pub struct Data {
        pub config: Config
    }

    #[derive(Deserialize)]
    #[derive(Serialize)]
    pub struct Config {
        pub secret: String
    }


    pub fn read_file(filename: &str) -> Data {
        let file_exists = check_exists(filename);
        if !file_exists{
            create_file(filename);
        }

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

    fn check_exists(filename: &str) -> bool{
        let result = match fs::metadata(filename) {
            Ok(_) => true,
            Err(_) => false,
        };
        return result;
    }
    
    fn create_file(filename: &str)  {
        let data: Data = Data { 
            config: Config { secret: "babayaga".to_string()}
        };
        let toml_data = toml::to_string(&data);

        let data = match toml_data {
            Ok(result) => result,
            Err(_) => {
                panic!("Unable to parse data to toml `{}`", filename);
            }
        };

        match fs::write(filename, data) {
            Ok(c) => c,
            Err(_) => {
                panic!("Could not write file `{}`", filename);
            }
        };
}
}