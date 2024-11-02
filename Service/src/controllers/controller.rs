use futures_util::stream::Stream;
use tokio::time::interval;
use bytes::Bytes;
use core::pin::Pin;
use core::time::Duration;
use actix_web::{get, web, HttpRequest, HttpResponse, Error};
use reqwest::Client;
use std::collections::HashMap;
use crate::messages::messages;

#[get("/event/{message}")]
async fn sse(_req: HttpRequest, info: web::Path<messages::Info>) -> Result<HttpResponse, Error> {
    // Extrair o token do cabeçalho de autorização
    let token = match _req.headers().get("Authorization") {
        Some(header_value) => header_value.to_str().ok().map(|s| s.trim_start_matches("Bearer ")),
        None => None,
    };

    // Verificar se o token foi fornecido
    let token = match token {
        Some(token) => token,
        None => return Ok(HttpResponse::Unauthorized().body("Missing authorization token")),
    };

    // Verificar a validade do token através do serviço externo
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("token", token.to_string());

    // Enviar a requisição ao serviço de autenticação
    let response = client
        .post("http://localhost:8000/api/v2/token-info")
        .json(&params)
        .send()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Authentication service unreachable"))?;

    // Validar a resposta do serviço de autenticação
    if !response.status().is_success() {
        return Ok(HttpResponse::Unauthorized().body("Invalid token"));
    }

    // Criar o stream SSE após a autenticação bem-sucedida
    let stream = async_stream::stream! {
        let mut interval = interval(Duration::from_secs(2));

        loop {
            interval.tick().await;
            let data = format!("data: {{ \"message\": \"Event {}\" }}\n\n", info.message);
            yield Ok::<_, actix_web::Error>(Bytes::from(data));
        }
    };

    Ok(
        HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<Bytes, actix_web::Error>>>>)
    )
}
