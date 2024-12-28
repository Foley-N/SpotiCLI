#![deny(clippy::all)]

//use reqwest;
//use dotenv;
use dotenv::dotenv;
use std::env;
//use std::String;
//use base64;

fn main() {
    let base_url = "https://api.spotify.com/v1/";
    println!("{}", base_url);
    get_token();
    search_album();
}

fn get_token() {
    dotenv().ok(); // reads .env

    let _client_secre = env::var("CLIENT_SECRET");
    let _client_id = env::var("CLIENT_ID");
    
    // match _client_id {
    //     Ok(val) => println!("CLIENT_SECRET: {:?}", val),
    //     Err(e) => println!("Error client secret: {}",e),
    // }

    //let auth_string: &str = client_id + ":" + client_secret;
    //println!("{} {}", client_id, client_secret);

    let _url = "https://accounts.spotify.com/api/token";
}

fn search_album() {
    let url = "https://api.spotify.com/v1/albums";
    println!("{}", url);
}
