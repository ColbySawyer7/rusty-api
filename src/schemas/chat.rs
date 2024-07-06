use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Prompt {
    pub question: String,
}

#[derive(Serialize)]
pub struct PromptResponse {
    pub question: String,
    pub answer: String,
}

#[derive(Serialize, Deserialize)]
pub struct Competitor{
    pub name: String,
    pub url: String,

}
#[derive(Serialize, Deserialize)]
pub struct CompetitorResponse{
    pub overview: String,
    pub strengths: String,
    pub weakensses: String,

}