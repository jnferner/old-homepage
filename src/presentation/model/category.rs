#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
pub struct Category {
    pub id: i32,
    pub text: String,
}
