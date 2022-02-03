use std::{fs::File,io::BufReader};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize,Deserialize)]
struct Token{
    token: String,
}

pub fn get_token(filename: &str)->Result<String>{
    let file=File::open(filename).unwrap();
    let reader=BufReader::new(file);
    let token: Token=serde_json::from_reader(reader).unwrap();

    Ok(token.token)
}