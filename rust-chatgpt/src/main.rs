use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env; 
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
} 
#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>, 
}
#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String, 
    max_tokens: u16,      
} 

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok(); 
    let https = HttpsConnector::new(); 
    let client = Client::builder().build(https); 
    let uri = "https://api.openai.com/v1/chat/completions";

    let preamble = "Generate a Sql code for the given statement."; 


    let oai_token: String = env::var("OPENAI_TOKEN").unwrap(); 
                                                            
    let auth_header_val = format!("Bearer {}", oai_token); 

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> "); 
        stdout().flush().unwrap();
        let mut user_text = String::new(); 
        stdin() 
            .read_line(&mut user_text)
            .expect("Failed to read line"); 

        println!("");

        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into()); 
        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text), 
            max_tokens: 1000, 
        };


        let body = Body::from(serde_json::to_vec(&oai_request)?);

       
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json") 
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

 
        let res = client.request(req).await?;
        
        let body = hyper::body::aggregate(res).await?; 
        let json: OAIResponse = serde_json::from_reader(body.reader())?;

   
        sp.stop();

        println!(""); 
        println!("{}", json.choices[0].text);
    }

    Ok(())
}