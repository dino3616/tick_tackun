use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::{Context,EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn ready(&self,_ctx: Context,ready: Ready){
        print!("{} is connected!",ready.user.name);
    }
}