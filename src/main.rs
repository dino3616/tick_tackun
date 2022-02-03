use std::process;
use serenity::{
    framework::{
        StandardFramework,
    },
    prelude::Client,
};

mod commands;
mod group;
mod handler;
mod tick_tackun;
mod token;

#[tokio::main]
async fn main(){
    let token=match token::get_token("config.json"){
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: serde_json said, {}\n",err);
            process::exit(1);
        }
    };

    let framework=StandardFramework::new()
        .configure(|c|c.with_whitespace(true).prefix("tick tackun:"))
        .help(&group::HELP)
        .group(&group::GENERAL_GROUP);

    let mut client=match Client::builder(&token)
        .event_handler(handler::Handler)
        .framework(framework)
        .await{
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error: serenity said, {}\n",err);
                process::exit(1);
            }
        };

    if let Err(err)=client.start().await{
        eprint!("Error: serenity said, {}\n",err);
        process::exit(1);
    }
}