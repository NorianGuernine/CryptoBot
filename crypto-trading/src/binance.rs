
mod binance {

    use std::fs::File;
    use std::path::Path;
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};
    use sha2::Sha256;
    use hmac::{Hmac, Mac};
    use reqwest;

    pub const PATH_KEY: &str = "keys/";
    const URL_ACCOUNT_INFO: &str = "https://testnet.binance.vision/api/v3/account?";
    
    type HmacSha256 = Hmac<Sha256>;

    #[derive(Deserialize, Debug)]
    struct KeysJson {
        api_key: String,
        secret_key: String,
    }     

    pub struct Binance {
        api_key: String,
        secret_key: String,
        signature_hex: String,
        timestamp_ms: u128,
    }

    impl Binance {
        pub fn new() -> Self {
            Self { 
                api_key: String::new(),
                secret_key: String::new(),
                signature_hex: String::new(),
                timestamp_ms : 0,
            }
        }

        pub fn get_api_key(&self) -> String {
            self.api_key.to_string()
        }

        pub fn get_secret_key(&self) -> String {
            self.secret_key.to_string()
        }

        pub fn read_keys(&mut self, path_file: &str) {
            let json_file_path = Path::new(path_file);
            let file = File::open(json_file_path).expect("Unable to open file");
            let reader:Vec<KeysJson> = serde_json::from_reader(file)
                .expect("error while reading or parsing");
            
            self.api_key = reader[0].api_key.clone();
            self.secret_key = reader[0].secret_key.clone();
        }

        pub async fn get_account_info(&mut self) -> Result<(), reqwest::Error> {
            self.calculate_timestamp_ms();
            self.generate_signature_request();
            let url = URL_ACCOUNT_INFO.to_string() 
                + &String::from("timestamp=") + &self.timestamp_ms.to_string()
                + &String::from("&signature=") + &self.signature_hex;
 
            let client = reqwest::Client::new();
 
            let res = client
                .get(&url)
                .header("X-MBX-APIKEY",self.api_key.as_str())
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            Ok(())

        }

        fn calculate_timestamp_ms(&mut self)  { 
            let start_time = SystemTime::now();
            let since_epoch = start_time.duration_since(UNIX_EPOCH)
                .expect("Time is before Unix epoch");
            self.timestamp_ms = since_epoch.as_millis();
        }

        fn generate_signature_request(&mut self)  {
            let query_str = String::from("timestamp=") + &self.timestamp_ms.to_string();
            let mut signature = HmacSha256::new_from_slice(self.secret_key.as_bytes())
                .expect("HMAC can take key of any size");
            signature.update(query_str.as_bytes());
    
            let result_sig = signature.finalize();
            self.signature_hex = hex::encode(result_sig.into_bytes());

        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_keys() {
        // call the function with the new file create
        let mut test_binance = binance::Binance::new();
        let file_key_path = binance::PATH_KEY.to_owned() + "test_false_keys_file.json";
        test_binance.read_keys(file_key_path.as_str());
        
        //value in file for api_key is ApiKeyValue and SecretKeyValue for secret_key
        assert_eq!(test_binance.get_api_key(), "ApiKeyValue");
        assert_eq!(test_binance.get_secret_key(), "SecretKeyValue");
    
    }

    #[test]
    #[should_panic(expected = "Unable to open file")]
    fn test_read_keys_no_file_found() {
        // call the function with the new file create
        let mut test_binance = binance::Binance::new();
        let file_key_path = binance::PATH_KEY.to_owned() + "test_false_key_file.json";
        test_binance.read_keys(file_key_path.as_str());
    }
    
    #[test]
    #[should_panic(expected = "error while reading or parsing")]
    fn test_read_keys_wrong_format() {
        // call the function with the new file create
        let mut test_binance = binance::Binance::new();
        let file_key_path = binance::PATH_KEY.to_owned() + "file_wrong_format.json";
        test_binance.read_keys(file_key_path.as_str());
    }

    #[test]
    fn test_get_request() {
        // call the function with the new file create
        let mut test_binance = binance::Binance::new();
        let file_key_path = binance::PATH_KEY.to_owned() + "test_api_key.json";
        test_binance.read_keys(file_key_path.as_str());
        test_binance.get_account_info();
    }
} 
