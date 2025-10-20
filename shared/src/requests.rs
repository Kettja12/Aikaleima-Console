use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request<'a> {
    pub login_token: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StampingRequest {
    pub id: i32,
    pub login_token: String,
    pub device_name: String,
    pub stamper_key: String,
    pub stampcategory_key: String,
    pub timestamp: String,
    pub latitude: String,
    pub longitude: String,
}
