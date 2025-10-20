#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Yritetään hakea login-token asetustiedostosta.
    // Jos sitä ei löydy, tulostetaan virhe ja poistutaan.
    let login_token = match shared_api::methods::get_setting("settings.txt", "login_token") {
        Some(token) => {
            println!("Käytetään login-tokenia asetustiedostosta.");
            token
        }
        None => {
            eprintln!("Virhe: login-tokenia ei löytynyt asetustiedostosta (settings.txt).\nLisää 'login_token=YOUR_TOKEN' tiedostoon ja yritä uudelleen.");
            return Ok(());
        }
    };

    let request = shared_api::requests::Request {
        login_token: &login_token
    };

    // Lue palvelimen osoite asetuksista, käytä oletusarvoa jos ei löydy.
    let server_url = shared_api::methods::get_setting("settings.txt", "server_url")
        .unwrap_or_else(|| "https://testi.aikaleima.fi".to_string());

    println!("Lähetetään data palvelimelle...");
    println!("Palvelin: {}", server_url);

    let response = reqwest::Client::new()
        .post(format!("{}/{}", server_url, shared_api::api_calls::STAMPS))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let stamps_response: shared_api::responces::StampsResponse = response.json().await?;
        println!("Vastaus palvelimelta jäsennetty onnistuneesti. Leimoja löytyi: {}", stamps_response.stamps.len());
        println!("{:#?}", stamps_response);

        // Tallennetaan leimat CSV-tiedostoon
        println!("\nTallennetaan leimat tiedostoon stamps.csv...");
        if let Err(e) = shared_api::methods::save_stamps_to_csv("stamps.csv", &stamps_response.stamps) {
            println!("Virhe CSV-tiedoston kirjoituksessa: {}", e);
        } else {
            println!("Leimat tallennettu onnistuneesti.");
        }
    } else {
        println!("Pyyntö epäonnistui, status: {}", response.status());
        let error_text = response.text().await?;
        println!("Virheen sisältö:\n{}", error_text);
    }

    Ok(())
}
