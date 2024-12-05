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
        // let mut cdp = client::CdpClient::custom("localhost", 9222)
        //     .connect_to_tab(0)
        //     .await
        //     .unwrap();
        //
        // // Send message to navigate to the URL
        // let response = cdp
        //     .send(
        //         "Network.getCookies",
        //         params!("urls" => vec!["https://www.google.com"]),
        //     )
        //     .await
        //     .unwrap();
        // println!("Response: {:?}", response);
    }
}
