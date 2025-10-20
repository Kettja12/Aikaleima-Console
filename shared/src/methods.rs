use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// Sisäinen funktio, joka lukee avain-arvo-pareja tiedostosta.
fn read_key_value_file(file_path: &str) -> io::Result<HashMap<String, String>> {
    let mut settings = HashMap::new();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // Ohitetaan tyhjät rivit ja kommentit (jotka alkavat #)
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            settings.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(settings)
}

/// Lukee annettua asetustiedostoa ja palauttaa arvon määritellylle avaimelle.
pub fn get_setting(file_path: &str, key: &str) -> Option<String> {
    match read_key_value_file(file_path) {
        Ok(settings) => settings.get(key).map(|s| s.to_string()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None, // Tiedostoa ei löytynyt, palautetaan None.
        Err(e) => {
            println!("Virhe lukiessa asetustiedostoa '{}': {}", file_path, e);
            None
        }
    }
}

/// Tallentaa avain-arvo-parin asetustiedostoon.
/// Säilyttää olemassa olevat asetukset ja päivittää/lisää vain annetun avaimen.
pub fn save_setting(file_path: &str, key: &str, value: &str) -> io::Result<()> {
    // Lue olemassa olevat asetukset, tai luo tyhjä kartta jos tiedostoa ei ole.
    let mut settings = match read_key_value_file(file_path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => HashMap::new(),
        Err(e) => return Err(e), // Palauta muut lukuvirheet
    };

    // Lisää tai päivitä uusi asetus.
    settings.insert(key.to_string(), value.to_string());

    // Kirjoita kaikki asetukset takaisin tiedostoon.
    let mut file = File::create(file_path)?;
    for (k, v) in settings.iter() {
        writeln!(file, "{}={}", k, v)?;
    }

    Ok(())
}

/// Tallentaa kaikki UserResponse-rakenteen kentät asetustiedostoon.
pub fn save_user_response_settings(file_path: &str, response: &super::responces::UserResponse) -> io::Result<()> {
    // Lue olemassa olevat asetukset, tai luo tyhjä kartta jos tiedostoa ei ole.
    let mut settings = match read_key_value_file(file_path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => HashMap::new(),
        Err(e) => return Err(e),
    };

    // Päivitä tai lisää kaikki kentät vastauksesta
    settings.insert("status".to_string(), response.status.clone());
    settings.insert("id".to_string(), response.id.to_string());
    settings.insert("name".to_string(), response.name.clone());
    settings.insert("login_token".to_string(), response.token.clone()); // Käytetään avainta "login_token"
    settings.insert("activated".to_string(), response.activated.to_string());
    settings.insert("pincode".to_string(), response.pincode.to_string());

    // Kirjoita kaikki asetukset takaisin tiedostoon.
    let mut file = File::create(file_path)?;
    for (k, v) in settings.iter() {
        writeln!(file, "{}={}", k, v)?;
    }

    Ok(())
}
/// Tallentaa kaikki DeviceResponse-rakenteen kentät asetustiedostoon.
pub fn save_device_response_settings(file_path: &str, response: &super::responces::DeviceResponse) -> io::Result<()> {
    // Lue olemassa olevat asetukset, tai luo tyhjä kartta jos tiedostoa ei ole.
    let mut settings = match read_key_value_file(file_path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => HashMap::new(),
        Err(e) => return Err(e),
    };

    // Päivitä tai lisää kaikki kentät vastauksesta
    settings.insert("status".to_string(), response.status.clone());
    settings.insert("id".to_string(), response.id.to_string());
    settings.insert("stamper_key".to_string(), response.stamper_key.clone());
    settings.insert("stampername".to_string(), response.stampername.clone()); 
    settings.insert("device_name".to_string(), response.device_name.to_string());
    settings.insert("device_token".to_string(), response.login_token.to_string());
    settings.insert("gps".to_string(), response.gps.to_string());

    // Kirjoita kaikki asetukset takaisin tiedostoon.
    let mut file = File::create(file_path)?;
    for (k, v) in settings.iter() {
        writeln!(file, "{}={}", k, v)?;
    }

    Ok(())
}

/// Tallentaa listan leimoja CSV-tiedostoon puolipisteellä erotettuna.
/// Kirjoittaa otsikkorivin ja ylikirjoittaa olemassa olevan tiedoston.
pub fn save_stamps_to_csv(file_path: &str, stamps: &[super::responces::StampResponse]) -> Result<(), Box<dyn std::error::Error>> {
    // Määritellään CSV-kirjoittaja käyttämään puolipistettä erottimena.
    let mut writer = ::csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path(file_path)?;

    // Kirjoitetaan kaikki leimat kerralla.
    // `csv`-kirjasto hoitaa otsikkorivin automaattisesti structin kenttien perusteella.
    for stamp in stamps {
        writer.serialize(stamp)?;
    }

    writer.flush()?;
    Ok(())
}