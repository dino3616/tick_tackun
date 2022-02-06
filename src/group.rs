use std::collections::HashSet;
use serenity::{
    framework::{
        standard::{
            Args,
            CommandGroup,
            CommandResult,
            help_commands,
            HelpOptions,
            macros::{group,help},
        },
    },
    model::{
        channel::Message,
        id::UserId,
    },
    prelude::Context,
};
use super::commands::show_words::*;

#[group]
#[commands(show_words)]
struct General;

#[help]
async fn help(ctx: &Context,msg: &Message,args: Args,help_options: &'static HelpOptions,groups: &[&'static CommandGroup],owners: HashSet<UserId>)->CommandResult{
    let _=help_commands::with_embeds(ctx,msg,args,help_options,groups,owners).await;

    Ok(())
}