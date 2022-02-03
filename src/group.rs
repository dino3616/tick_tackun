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
use super::commands::hello::*;

#[group]
#[summary("一般")]
#[commands(hello)]
struct General;

#[help]
#[individual_command_tip="こいつはヘルプコマンド。こいつはケン。そしてこの御方はリュウセイさん。"]
async fn help(ctx: &Context,msg: &Message,args: Args,help_options: &'static HelpOptions,groups: &[&'static CommandGroup],owners: HashSet<UserId>)->CommandResult{
    let _=help_commands::with_embeds(ctx,msg,args,help_options,groups,owners).await;

    Ok(())
}