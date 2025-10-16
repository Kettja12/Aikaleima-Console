use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request<'a> {
    pub login_token: &'a str,
}
