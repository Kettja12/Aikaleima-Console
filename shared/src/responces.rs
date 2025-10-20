use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct DeviceResponse {
    pub status: String,
    pub id: i32,
    pub stamper_key: String,
    pub stampername: String,
    pub device_name: String,
    pub login_token: String,
    pub activated: bool,
    pub gps: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct UserResponse {
    pub status: String,
    pub id: i32,
    pub name: String,
    pub token: String,
    pub activated: bool,
    pub pincode: bool,
}


#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StampResponse {
    pub id: String,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub status: String,
}