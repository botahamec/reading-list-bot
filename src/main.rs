#![feature(never_type)]
#![feature(exhaustive_patterns)]
#![feature(destructuring_assignment)]
#![feature(const_fn_trait_bound)]

use log::LevelFilter;
use managers::secrets::SecretManager;
use serenity::{
	async_trait,
	client::{Context, EventHandler},
	framework::StandardFramework,
	model::{channel::Message, prelude::Ready},
	Client,
};
use simple_logger::SimpleLogger;

mod managers;
mod services;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	// Set a handler for the `message` event - so that whenever a new message
	// is received - the closure (or function) passed will be called.
	//
	// Event handlers are dispatched through a threadpool, and so multiple
	// events can be dispatched simultaneously.
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content == "!ping" {
			// Sending a message can fail, due to a network error, an
			// authentication error, or lack of permissions to post in the
			// channel, so log to stdout when some error happens, with a
			// description of it.
			if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
				println!("Error sending message: {:?}", why);
			}
		}
	}

	// Set a handler to be called on the `ready` event. This is called when a
	// shard is booted, and a READY payload is sent by Discord. This payload
	// contains data like the current user's guild Ids, current user data,
	// private channels, and more.
	//
	// In this case, just print what the current user's username is.
	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	SimpleLogger::new()
		.with_level(LevelFilter::Warn)
		.with_module_level("reading-list-bot", LevelFilter::Trace)
		.init()
		.unwrap();

	let secret_manager = SecretManager::default();

	let framework = StandardFramework::new();

	// Configure the client with your Discord bot token in the environment.
	let token = secret_manager
		.discord_token()
		.expect("Expected a token in the environment");

	// Create a new instance of the Client, logging in as a bot. This will
	// automatically prepend your bot token with "Bot ", which is a requirement
	// by Discord for bot users.
	let mut client = Client::builder(token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Err creating client");

	// Finally, start a single shard, and start listening to events.
	//
	// Shards will automatically attempt to reconnect, and will perform
	// exponential backoff until it reconnects.
	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}
