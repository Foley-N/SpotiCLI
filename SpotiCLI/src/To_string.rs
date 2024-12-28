#![deny(clippy::all)]
use std::String;

pub struct Credentials {
    client_id = String,
    client_secret: String,
}

impl Credentials {
    pub fn new(client_id: String, client_secret: String) -> self {
        Credentials{
            client_id,
            client_secret,
        }
    }

    pub fn to_string(&self) -> String {
        // we don't want to disclose the secret
        format!("Credentials({})", $self.client_id);   
    }
}