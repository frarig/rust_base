use std::time::Duration;
use reqwest::blocking::Client;

pub struct Word {
    client: Client,
}

impl Word {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .connect_timeout(Duration::from_secs(1))
            .build()
            .unwrap();
        
        Word { client }
    }

    pub fn get_word_by_word_len(&self, word_len: &i32) -> Vec<String> {
        let url = format!("https://random-word-api.herokuapp.com/word?number=1&length={}", word_len);
        let default = vec![String::from("rust")];

        self.client.get(url)
            .send()
            .and_then(|r| r.error_for_status())
            .and_then(|r| r.json::<Vec<String>>())
            .unwrap_or(default)
    }
}

