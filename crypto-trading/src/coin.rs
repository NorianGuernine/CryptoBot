pub mod coin {
	pub struct Coin {
		coin_name: String,
	}
	
	impl Coin {
		pub fn new(coin_name: String) -> Self {
            Self { coin_name }
        }
	}
}