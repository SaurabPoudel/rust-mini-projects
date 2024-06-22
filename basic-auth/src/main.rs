use std::env;
use std::error::Error;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: email passwd, leave password if none");
        std::process::exit(1);
    }

    let client = Client::new();
    let user = args[1].clone();
    let passwd = if args.len() > 2 { Some(args[2].clone()) } else { None };

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send()?;

    println!("{:?}", response.text()?);
    Ok(())
}
