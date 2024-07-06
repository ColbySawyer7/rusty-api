use actix_web::{
    error::Error, web, HttpResponse, Responder,
};
use crate::schemas::chat::{Prompt, PromptResponse};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};


#[actix_web::post("/chat")]
pub async fn ask_chat(
    prompt: web::Json<Prompt>,
) -> Result<impl Responder, Error> {
    let question = &prompt.question;

    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "llama3:latest".to_string();

    match ollama
        .generate(GenerationRequest::new(model, question.clone()))
        .await
    {
        Ok(res) => {
            let response = PromptResponse {
                question: question.clone(),
                answer: res.response,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(_) => Err(actix_web::error::ErrorNotFound("Internal Service Error: Please try again")),
    }
}

