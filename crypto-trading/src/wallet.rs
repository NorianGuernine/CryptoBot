mod wallet {
	use std::vec::Vec;
	use serde_json;
	use crate::coin::coin::Coin;
	use crate::binance::binance::Binance;
	struct Wallet {
		total_balance: f64,
		crypto_list: Vec<Coin>,
		raw_wallet_binance: serde_json::Value,
	}
	
	impl Wallet {
		pub fn new() -> Self {
			Self {
				total_balance: 0.0,
				crypto_list: Vec::new(),
				raw_wallet_binance: serde_json::Value::Null,
			}
		}
		
		pub async fn update(&mut self) -> Result<(), reqwest::Error> {
			let mut binance_api = Binance::new();
			self.raw_wallet_binance = binance_api.get_account_info().await?;
			Ok(())
		}
		
		pub fn add(&mut self, coin_name: &str) {
			let coin_to_add = Coin::new(coin_name.to_string());
			//check if coin exist in binance
			
			
			self.crypto_list.push(coin_to_add);
		}
	}
}