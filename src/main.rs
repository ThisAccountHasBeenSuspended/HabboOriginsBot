#[macro_use]
extern crate log;
#[macro_use]
extern crate bson;

use core::panic;

// CRATE
use serenity::{
    all::{ActivityData, OnlineStatus},
    prelude::{Client, GatewayIntents},
};
use tokio::runtime::{Builder, Runtime};

// MOD
mod commands;
mod helper;
mod mongo;
mod structs;

pub const LOOKUP_URL: &str = "https://origins.habbo.com/api/public/users?name=";

pub fn settings() -> &'static structs::Settings {
    use std::sync::OnceLock;
    static VAL: OnceLock<structs::Settings> = OnceLock::new();
    VAL.get_or_init(structs::Settings::load)
}

fn custom_panic() {
    std::panic::set_hook(Box::new(|panic_info| {
        let mut msg = String::from("(panic occurred)");

        if let Some(l) = panic_info.location() {
            let loc_msg = format!(" | file '{}' at line {}", l.file(), l.line());
            msg.push_str(&loc_msg);
        }

        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            msg.push_str("\n> ");
            msg.push_str(s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            msg.push_str("\n> ");
            msg.push_str(s);
        }

        error!("{}", msg);

        std::process::exit(1);
    }));
}

pub fn custom_runtime() -> Runtime {
    let settings_threads = settings().get_threads();
    match Builder::new_multi_thread()
        .worker_threads(settings_threads.get_count() as usize)
        .thread_name("origins-runtime-worker") // Default: "tokio-runtime-worker"
        .thread_stack_size(settings_threads.get_stack_size()) // Default: 2 MiB
        .enable_io()
        .enable_time()
        .build()
    {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    if log4rs::init_file("logging.yaml", Default::default()).is_err() {
        panic!("`logging_config.yaml` is missing");
    }

    custom_panic();

    custom_runtime().block_on(async {
        start().await;
    });
}

pub async fn start() {
    println!("Connecting to MongoDB...");
    mongo::init().await;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;

    let token = settings().get_guild().get_token();
    let mut client = match Client::builder(token, intents)
        .event_handler(structs::Handler)
        .activity(ActivityData::playing("Habbo Hotel:Origins"))
        .status(OnlineStatus::DoNotDisturb)
        .await
    {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    println!("Bot started!");
    if let Err(e) = client.start().await {
        panic!("{:?}", e);
    }
}
