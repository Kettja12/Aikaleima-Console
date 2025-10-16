use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DeviceResponse {
    pub status: String,
    pub id: i32,
    // JSON-kenttä on "name", yhdistetään se tähän.
    pub name: String,
    // JSON-kenttä on "token", yhdistetään se tähän.
    pub token: String,
    pub activated: bool,
    pub pincode: bool,
}


#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StampResponse {
    pub id: i32,
    pub device_key: String,
    pub stamper_key: String,
    pub stamper_name: String,
    pub stampcategory_key: String,
    pub stampcategory_name: String,
    pub timestamp: String,
    pub state: String,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Deserialize, Debug)]
pub struct StampsResponse {
    pub status: Option<String>,
    pub stamps: Vec<StampResponse>,
}