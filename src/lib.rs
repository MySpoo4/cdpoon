pub mod client;
pub mod error;
pub mod macros;
pub mod models;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(run());
    }

    async fn run() {
        let mut client = client::CdpClient::custom("localhost", 9222);

        // Send message to navigate to the URL
        let response = client
            .connect_to_tab(0)
            .await
            .unwrap()
            .send(models::Cmd {
                method: "Page.navgiate",
                params: params!("url" => "https://www.starbucks.com"),
            })
            .await
            .unwrap();

        println!("Response: {:?}", response);

        let response2 = client
            .send(models::Cmd {
                method: "Runtime.evaluate",
                params: params!("expression" => "$('#primary-content')"),
            })
            .await
            .unwrap();

        println!("Response: {:?}", response2);
    }
}
