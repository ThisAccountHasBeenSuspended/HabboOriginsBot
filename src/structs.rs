use std::fs::File;

use serde::{Deserialize, Serialize};
use serenity::{
    all::{GuildId, Interaction, Ready},
    async_trait,
    prelude::{Context, EventHandler},
};

use crate::helper;

#[derive(Deserialize)]
pub struct Guild {
    id: u64,
    token: Box<str>,
    verify_role_id: u64,
}

impl Guild {
    #[inline(always)]
    pub fn get_id(&'static self) -> u64 {
        self.id
    }

    #[inline(always)]
    pub fn get_token(&'static self) -> &'static str {
        &self.token
    }

    #[inline(always)]
    pub fn get_verify_role_id(&'static self) -> u64 {
        self.verify_role_id
    }
}

#[derive(Deserialize)]
pub struct Threads {
    count: u16,
    stack_size: usize,
}

impl Threads {
    #[inline(always)]
    pub fn get_count(&'static self) -> u16 {
        self.count
    }

    #[inline(always)]
    pub fn get_stack_size(&'static self) -> usize {
        self.stack_size
    }
}

#[derive(Deserialize)]
pub struct Settings {
    mongodb_uri: Box<str>,
    guild: Guild,
    threads: Threads,
}

impl Settings {
    pub fn load() -> Self {
        let file = match File::open("settings.json") {
            Ok(r) => r,
            Err(e) => panic!("`settings.json` is missing\n> {}", e),
        };

        let data = match std::io::read_to_string(file) {
            Ok(r) => r,
            Err(e) => panic!("`settings.json` could not be read\n> {}", e),
        };

        match serde_json::from_str(&data) {
            Ok(r) => r,
            Err(e) => panic!("`settings.json` could not be deserialized\n> {}", e),
        }
    }

    #[inline(always)]
    pub fn get_mongodb_uri(&'static self) -> &'static str {
        &self.mongodb_uri
    }

    #[inline(always)]
    pub fn get_guild(&'static self) -> &'static Guild {
        &self.guild
    }

    #[inline(always)]
    pub fn get_threads(&'static self) -> &'static Threads {
        &self.threads
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, ia: Interaction) {
        if let Interaction::Command(command) = ia {
            let result = match command.data.name.as_str() {
                "verify" => crate::commands::verify::run(&ctx.http, &command).await,
                "check" => crate::commands::check::run(&ctx.http, &command).await,
                "reset" => crate::commands::reset::run(&ctx.http, &command).await,
                "info" => crate::commands::info::run(&ctx.http, &command).await,
                _ => "Oops!".into()
            };
            helper::edit_reply(&ctx.http, result, &command).await;
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id = GuildId::new(crate::settings().get_guild().get_id());

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    crate::commands::verify::register(),
                    crate::commands::check::register(),
                    crate::commands::reset::register(),
                    crate::commands::info::register(),
                ],
            )
            .await;
    }
}

#[derive(Deserialize)]
pub struct Badge {
    pub code: Box<str>,
    pub name: Box<str>,
}

#[derive(Default, Deserialize)]
pub struct Profile {
    #[serde(rename = "uniqueId")]
    pub unique_id: Box<str>,
    #[serde(rename = "figureString")]
    pub figure_string: Box<str>,
    pub motto: Box<str>,
    pub online: bool,
    #[serde(rename = "lastAccessTime")]
    pub last_access_time: Box<str>,
    #[serde(rename = "memberSince")]
    pub member_since: Box<str>,
    #[serde(rename = "selectedBadges")]
    pub selected_badges: Vec<Badge>,
}

#[derive(Serialize, Deserialize)]
pub struct VerifiedUser {
    pub id: String,
    pub habbo: Box<str>,
    pub verified: bool,
}
