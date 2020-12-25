use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello Earthlings, We are here to destroy you! Just kidding 🤖

You have summoned me. Let's see about getting you what you need.

❓ Need technical help?
➡️ Post in the <#790100334525677571> channel and other humans will assist you.

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://github.com/yildizozan/geveze>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that resolves your issue!

— Geveze 🤖
";

const HELP_COMMAND1: &str = "!help";
const HELP_COMMAND2: &str = "!eyruuh";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND1 || msg.content == HELP_COMMAND2 {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
