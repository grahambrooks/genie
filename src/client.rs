use futures::StreamExt;

pub(crate) struct Client {
    client: reqwest::Client,
    token: String,
}

impl Client {
    pub(crate) fn new(token: String) -> Client {
        Client {
            client: reqwest::Client::new(),
            token: token,
        }
    }

    pub(crate) async fn call(&self, _prompt: String, callback: fn(String)) {
        self.client.get("https://blog.grahambrooks.dev").send().await.unwrap();
        let mut stream = reqwest::get("https://blog.grahambrooks.dev").await.unwrap().bytes_stream();
        let mut buffer = Vec::new();
        while let Some(item) = stream.next().await {
            match item {
                Ok(item) =>
                    for byte in item {
                        if byte == b'\n' {
                            callback(String::from_utf8(buffer.clone()).unwrap());
                            buffer.clear();
                        } else {
                            buffer.push(byte);
                        }
                    }
                Err(e) => println!("Error: {}", e),
            };
        }
    }
}

