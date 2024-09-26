use std::sync::Arc;

use serenity::all::{
    builder::CreateCommand, Colour, CommandInteraction, CommandOptionType, CreateCommandOption,
    CreateEmbed, CreateMessage, Http,
};

pub async fn run(http: &Arc<Http>, interaction: &CommandInteraction) -> String {
    if interaction.data.options.is_empty() {
        return format!(
            "Hello <@{}> :)\n\nThe username is missing!",
            interaction.user.id,
        );
    }

    let habbo = interaction.data.options[0].value.as_str().unwrap();

    let url = format!("{}{}", crate::LOOKUP_URL, habbo);

    let (req_status, req_result) = crate::helper::reqwest(&url, |response| async {
        let text = response.text().await.unwrap_or_default();
        serde_json::from_str::<serde_json::Value>(&text).unwrap_or_default()
    })
    .await;

    if !req_status {
        return format!("Hello <@{}> :)\n\nThe Habbo Hotel:Origins request has failed! Please try again later!", interaction.user.id);
    }

    let raw_res_value = req_result.unwrap().await;
    if let Some(res_error) = raw_res_value.get("error") {
        return format!(
            "Hello <@{}> :)\n\nThe Habbo \"{}\" does not exist or the profile has been set to private!\n\n**error:**\n`{}`",
            interaction.user.id,
            habbo,
            res_error.as_str().unwrap()
        );
    }

    let res_value = serde_json::from_value::<crate::structs::Profile>(raw_res_value).unwrap_or_default();

    let thumbnail = format!(
        "https://www.habbo.com/habbo-imaging/avatarimage?size=l&figure={}&size=b&direction=4&head_direction=4&crr=0&gesture=sml&frame=1",
        res_value.figure_string
    );
    let mut embed = CreateEmbed::new()
        .color(Colour::GOLD)
        .thumbnail(thumbnail)
        .title(habbo)
        .field("id", res_value.unique_id, false)
        .field("figure", res_value.figure_string, false)
        .field("motto", res_value.motto, false)
        .field("online", res_value.online.to_string(), false)
        .field("member since", res_value.member_since, true)
        .field("last login", res_value.last_access_time, true);

    let mut badges = Vec::with_capacity(res_value.selected_badges.len());
    for badge in &res_value.selected_badges {
        let badge_result = format!("[{}] {}", badge.code, badge.name);
        badges.push(badge_result);
    }
    embed = embed.field("badges", badges.join("\n"), false);

    let msg_content = format!(
        "<@{}>, here is your requested information about this Habbo.",
        interaction.user.id
    );
    let msg = CreateMessage::new().content(msg_content).add_embed(embed);
    let _ = interaction.channel_id.send_message(http, msg).await;

    format!(
        "Hello <@{}> :)\n\nHere is your information about the Habbo `{}` :)",
        interaction.user.id,
        habbo
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info")
        .description("Get informations about a Habbo")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "username",
                "The name of the Habbo",
            )
            .required(true),
        )
}
