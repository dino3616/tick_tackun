use std::process;
use serenity::{
    async_trait,
    model::channel::Message,
    prelude::{Context,EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx: Context, msg: Message){
        if msg.author.bot==true|!is_tingling_words(&msg.content){
            return;
        }

        if let Err(err)=msg.author.direct_message(&ctx,|m|m.content(&msg.content)).await{
            eprint!("Error: serenity said, {}\n",err);
            process::exit(1);
        }
    }
}

fn is_tingling_words(_word: &String)->bool{
    true
}