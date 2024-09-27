use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};
use serenity::{
    all::{GuildId, Interaction, Ready},
    async_trait,
    prelude::{Context, EventHandler},
};

#[derive(Deserialize, Serialize)]
pub struct MongoDB {
    uri: Box<str>,
    database: Box<str>,
}

impl MongoDB {
    #[inline(always)]
    pub fn get_uri(&'static self) -> &'static str {
        &self.uri
    }

    #[inline(always)]
    pub fn get_database(&'static self) -> &'static str {
        &self.database
    }
}

#[derive(Deserialize, Serialize)]
pub struct Guild {
    id: u64,
    #[serde(default)]
    verify_role_id: u64,
}

impl Guild {
    #[inline(always)]
    pub fn get_id(&'static self) -> u64 {
        self.id
    }

    #[inline(always)]
    pub fn get_verify_role_id(&'static self) -> u64 {
        self.verify_role_id
    }

    #[inline(always)]
    pub fn set_verify_role_id(&'static mut self, val: u64) {
        self.verify_role_id = val;
    }
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Settings {
    mongodb: MongoDB,
    guild: Guild,
    token: Box<str>,
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
    pub unsafe fn as_mut(&'static self) -> &'static mut Self {
        crate::helper::ref_to_refmut(&self)
    }

    #[inline(always)]
    pub fn get_mongodb(&'static self) -> &'static MongoDB {
        &self.mongodb
    }

    #[inline(always)]
    pub fn get_guild(&'static self) -> &'static Guild {
        &self.guild
    }

    #[inline(always)]
    pub unsafe fn get_guild_mut(&'static mut self) -> &'static mut Guild {
        &mut self.guild
    }

    #[inline(always)]
    pub fn get_token(&'static self) -> &'static str {
        &self.token
    }

    #[inline(always)]
    pub fn get_threads(&'static self) -> &'static Threads {
        &self.threads
    }

    pub fn save(&'static self) {
        let buf = serde_json::to_vec_pretty(&self).unwrap();
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("settings.json")
            .unwrap();
        let _ = file.write_all(&buf);
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, ia: Interaction) {
        if let Interaction::Command(command) = ia {
            let reply_msg = format!("Hello <@{}> :)\n\nRunning ...", command.user.id);
            crate::helper::reply(&ctx.http, reply_msg, &command).await;

            let result = match command.data.name.as_str() {
                "init" => crate::commands::init::run(&command).await,
                "verify" => crate::commands::verify::run(&ctx.http, &command).await,
                "check" => crate::commands::check::run(&command).await,
                "reset" => crate::commands::reset::run(&ctx.http, &command).await,
                "info" => crate::commands::info::run(&ctx.http, &command).await,
                _ => "Oops!".into()
            };
            crate::helper::edit_reply(&ctx.http, result, &command).await;
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id = GuildId::new(crate::settings().get_guild().get_id());

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    crate::commands::init::register(),
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
