use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthenticationResult {
    access_token: String,    
    expires_in: i32,    
    id_token: String,
    refresh_token: String,
    token_type: String    
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ChallengeParameters {
    challenge_name: Option<String>,
    challenge_parameters: Option<String>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ResultData {
    authentication_result: AuthenticationResult,
    challenge_parameters: ChallengeParameters
}



// A macro #[tokio::main] é usada para indicar que a função main é assíncrona
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nAPI-GET\n");

    // Cria um cliente HTTP
    let client = reqwest::Client::new();
    
    let url = "https://bk07exvx19.execute-api.us-east-1.amazonaws.com/dev-stage/oauth/login";
    
    // Cria um objeto JSON com os dados de autenticação
    let body = json!({
        "clientId": "3veb9e18d50ceqes38o1i8mlph",
        "username": "christian@payer.com.br",
        "password": "Panvel@2024"
    });
    
    // Cria um objeto HeaderMap para armazenar os cabeçalhos da requisição
    let mut headers = HeaderMap::new();
    
    // Adiciona o cabeçalho Content-Type: application/json
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    // Envia a requisição POST para a URL especificada        
    let response = client
        .post(url)        // Define o método HTTP como POST
        .headers(headers) // Adiciona os cabeçalhos da requisição
        .json(&body)      // Adiciona o corpo da requisição
        .send()           // Envia a requisição
        .await?;          // Aguarda a resposta do servidor
    
    // Verifica se a requisição retornou um status de erro
    let response_text = response.text().await?;
    
    // Faz o parse do JSON retornado pelo servidor
    let result_data: ResultData = serde_json::from_str(&response_text).expect("Erro ao fazer parse do JSON");

    println!("BeraerToken: {}", result_data.authentication_result.id_token);

    //let response_text = response.text().await?;
    //println!("Resposta: {}", response_text);
    
    Ok(())
}