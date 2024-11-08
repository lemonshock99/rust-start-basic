use reqwest::Error;
use std::collections::HashMap;

#[tokio::main]
pub async fn getip() -> Result<(), Error> {

    // Replace with your PHPIPAM server details and token
    let base_url = "http://127.0.0.1:9080/api/test01"; // Adjust 'my_app' to your API app name
    let token = "UbhqhoekF-XH3Y1SDP9C_JBIfZuokPNg";
    // let subnet_id = "3"; // Replace with the specific subnet ID you want to retrieve

    // Build the URL
    let url = format!("{}/sections/", base_url);

    // Make the GET request with headers
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
        .header("token", token)
        .send()
        .await?;

    // Check if the request was successful

    if response.status().is_success() {
        // Parse the JSON response
        let subnet_data: HashMap<String, serde_json::Value> = response.json().await?;
        println!("Subnet Details: {:?}", subnet_data);
    } else {
        eprintln!("Error: {}", response.status());
    }

    Ok(())
}