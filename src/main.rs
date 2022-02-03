use serenity::{
    framework::{
        StandardFramework,
    },
    prelude::Client,
};

mod commands;
mod group;
mod handler;
mod token;

#[tokio::main]
async fn main(){
    let token=token::get_token("config.json").expect("Error: トークンが見つかりませんでした.");

    let framework=StandardFramework::new()
        .configure(|c|c.prefix("tick tackun: "))
        .help(&group::HELP)
        .group(&group::GENERAL_GROUP);

    let mut client=Client::builder(&token)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Error: クライアントを作成できませんでした.");

    if let Err(err)=client.start().await{
        print!("Error: {}",err);
    }
}