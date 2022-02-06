use std::process;
use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::prelude::*,
    prelude::*,
};
use crate::sql::*;

#[command]
async fn show_words(ctx: &Context,msg: &Message)->CommandResult{
    let mut database=match DataBase::new().await{
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: {}\n",err);
            process::exit(1);
        }
    };
    if let Err(err)=database.fetch_all().await{
        eprint!("Error: {}\n",err);
        process::exit(1);
    };

    for word in database.words{
        msg.channel_id
            .say(&ctx.http,format!("{}",word))
            .await?;
    }

    Ok(())
}