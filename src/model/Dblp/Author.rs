#[derive(serde::Deserialize, Debug, Clone)]
pub struct Author{
    #[serde(rename = "@pid")]
    pub pid: String,
    pub text: String
}