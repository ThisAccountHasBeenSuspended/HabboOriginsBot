use std::sync::Arc;

use mongodb::Collection;
use rand::Rng;
use serenity::{
    all::{
        CommandInteraction, GuildId, Http, RoleId, UserId,
    },
    builder::CreateCommand,
    futures::TryStreamExt,
};

use crate::structs::VerifiedUser;

async fn check(interaction: &CommandInteraction, coll: &Collection<VerifiedUser>) -> bool {
    let query = doc! {
        "id": interaction.user.id.to_string(),
        "verified": true,
    };
    match coll.find_one(query).await {
        Ok(r) => r.is_some(),
        Err(_) => false,
    }
}

async fn add(habbo: &str, interaction: &CommandInteraction, coll: &Collection<VerifiedUser>) -> bool {
    let verified_user = VerifiedUser {
        id: interaction.user.id.to_string(),
        habbo: habbo.into(),
        verified: false,
    };
    if let Err(e) = coll.insert_one(verified_user).await {
        error!("{}", e);
        return false;
    }
    true
}

async fn delete(
    http: &Arc<Http>,
    habbo: &str,
    interaction: &CommandInteraction,
    coll: &Collection<VerifiedUser>,
    guild_id: GuildId,
    role_id: RoleId,
) {
    let query = doc! {
        "id": {
            // "$ne" is a filter.
            // It will select all documents but not the one with the following id.
            "$ne": interaction.user.id.to_string(),
        },
        "habbo": habbo,
    };

    // Remove roles from users.
    if let Ok(mut users) = coll.find(query.clone()).await {
        'userloop: while let Ok(u_o) = users.try_next().await {
            let uid = if let Some(u) = u_o {
                u.id.parse::<u64>().unwrap()
            } else {
                break 'userloop;
            };
            let uid = UserId::from(uid);
            let _ = http.remove_member_role(guild_id, uid, role_id, None).await;
        }
    }

    // Delete all users from the database. 
    let _ = coll.delete_many(query).await;
}

async fn update(habbo: &str, interaction: &CommandInteraction, coll: &Collection<VerifiedUser>) -> bool {
    let (first_query, second_query) = (
        doc! {
            "id": interaction.user.id.to_string(),
            "habbo": habbo,
        },
        doc! {
            "$set": {
                "verified": true,
            }
        }
    );

    if let Err(e) = coll.update_one(first_query, second_query).await {
        error!("{}", e);
        return false;
    }

    true
}

pub async fn run(http: &Arc<Http>, interaction: &CommandInteraction) -> String {
    crate::check_role_available!(http, interaction.user.id.get());

    if interaction.data.options.is_empty() {
        return format!(
            "Hello <@{}> :)\n\nThe username is missing!",
            interaction.user.id,
        );
    }

    let habbo = interaction.data.options[0].value.as_str().unwrap();
    let coll = crate::mongo::get_coll("verified_users");

    if check(interaction, &coll).await {
        return format!("Hello <@{}> :)\n\nYou are already verified! Use the command `/reset` to delete all your data from our database, remove all your roles and verify yourself again.", interaction.user.id);
    }

    let verify_code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    if !add(habbo, interaction, &coll).await {
        return format!("Hello <@{}> :)\n\nUnfortunately we could not add you to our database! Please try again later!", interaction.user.id);
    } else {
        let reply_msg = format!("Hello <@{}> :)\n\nTo verify yourself, change your motto to `{}` within the next 45 seconds and change it again after a successful verification!", interaction.user.id, verify_code);
        crate::helper::edit_reply(&http, reply_msg, interaction).await;
    }

    // Wait 45 seconds ...
    tokio::time::sleep(tokio::time::Duration::from_secs(45)).await;

    // Retrieve Habbo profile data
    let url = format!("{}{}", crate::LOOKUP_URL, habbo);
    let (req_status, req_result) = crate::helper::reqwest(&url, |res| async {
        let res_text = res.text().await.unwrap_or_default();
        serde_json::from_str::<serde_json::Value>(&res_text).unwrap_or_default()
    }).await;

    if !req_status {
        return format!("Hello <@{}> :)\n\nThe Habbo Hotel:Origins request has failed! Please try again later!", interaction.user.id);
    }

    let res_value = req_result.unwrap().await;

    if let Some(ev) = res_value.get("error") {
        return format!(
            "Hello <@{}> :)\n\nThe Habbo \"{}\" does not exist or the profile has been set to private!\n\n**error:**\n`{}`",
            interaction.user.id,
            habbo,
            ev.as_str().unwrap(),
        );
    }

    if res_value["motto"] != verify_code {
        return format!(
            "Hello <@{}> :)\n\nThe motto of the Habbo \"{}\" was not changed to `{}` within 45 seconds. Verification failed!",
            interaction.user.id,
            habbo,
            verify_code,
        );
    }

    let settings = crate::settings();
    let guild_id = settings.get_guild().get_id().into();
    let role_id = settings.get_guild().get_verify_role_id().into();

    delete(&http, habbo, interaction, &coll, guild_id, role_id).await;

    if !update(habbo, interaction, &coll).await {
        return format!("Hello <@{}> :)\n\nUnfortunately we could not update your data in our database! Please try again later!", interaction.user.id);
    }

    let _ = http
        .add_member_role(guild_id, interaction.user.id, role_id, None)
        .await;

    format!(
        "Hello <@{}> :)\n\nCongratulations! You have successfully verified yourself!",
        interaction.user.id
    )
}

pub fn register() -> CreateCommand {
    use serenity::all::{CreateCommandOption, CommandOptionType};

    CreateCommand::new("verify")
        .description("Verify your account")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "username",
                "The name of your Habbo",
            )
            .required(true),
        )
}
