use std::process;
use serenity::{
    async_trait,
    model::channel::Message,
    prelude::{Context,EventHandler},
};
use indoc::formatdoc;
use super::tick_tackun::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx: Context, msg: Message){
        if msg.author.bot==true|!TickTackun::is_tingling_words(&msg.content){
            return;
        }

        let warning_message=formatdoc!{"
            あなたは「チクチク言葉」を発言してしまったかもしれません。
            誰かがスマイルじゃなくなるような表現は避けてください。
            あなたと、私と、それとみんなのスマイルのためです。

            You may have uttered the tingling words.
            Please don't say anything that would make someone not smile.
            It is for yours, mine, and everyone else's smile.

            {}"
            ,msg.link()
        };

        if let Err(err)=msg.author.direct_message(&ctx,|m|m.content(warning_message)).await{
            eprint!("Error: serenity said, {}\n",err);
            process::exit(1);
        }
    }
}