use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

#[command]
#[description="ガラルヤドンのように鳴く(えぇ..."]
async fn hello(ctx: &Context,msg: &Message)->CommandResult{
    msg.channel_id
        .say(&ctx.http,format!("{} ゃ^～ん",msg.author.mention()))
        .await?;

    Ok(())
}