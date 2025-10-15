use std::io;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Request<'a> {
    login_token: &'a str
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Syötä login-token:");

    let mut login_token_input = String::new();

    io::stdin()
        .read_line(&mut login_token_input)
        .expect("Rivinvaihtoa ei voitu lukea");

    let login_token = login_token_input.trim();


    let request = Request {
        login_token: login_token
    };

    println!("Lähetetään data palvelimelle...");

    let response = reqwest::Client::new()
        .post(format!("https://testi.aikaleima.fi/{}", shared_api::USER_LOG_IN))
        .json(&request)
        .send()
        .await?;

    println!("Vastaus palvelimelta:");
    println!("{:#?}", response);

    let response_text = response.text().await?;
    println!("Vastauksen sisältö:\n{}", response_text);

    Ok(())
}
