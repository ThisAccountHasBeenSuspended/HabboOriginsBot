use serenity::{
    all::CommandInteraction,
    builder::CreateCommand,
};

use crate::structs::VerifiedUser;

pub async fn run(interaction: &CommandInteraction) -> String {
    if interaction.data.options.is_empty() {
        return format!(
            "Hello <@{}> :)\n\nThe user is missing!",
            interaction.user.id,
        );
    }

    if let Some(cmd_data) = interaction.data.options.first() {
        let user_id = match cmd_data.value.as_user_id() {
            Some(r) => r,
            None => {
                return format!(
                    "Hello <@{}> :)\n\nPlease use a valid user!",
                    interaction.user.id,
                );
            }
        };

        let coll = crate::mongo::get_coll::<VerifiedUser>("verified_users");
        let query = doc! {
            "id": user_id.get().to_string(),
            "verified": true,
        };

        if let Ok(user_o) = coll.find_one(query).await {
            if let Some(user) = user_o {
                return format!(
                    "Hello <@{}> :)\n\nThe user <@{}> is verified as Habbo `{}`!",
                    interaction.user.id,
                    user.id,
                    user.habbo,
                );
            }

            return format!(
                "Hello <@{}> :)\n\nThe user <@{}> is not verified!",
                interaction.user.id,
                user_id.get(),
            );
        }
    }

    format!(
        "Hello <@{}> :)\n\nSomething went wrong! Please try again later!",
        interaction.user.id,
    )
}

pub fn register() -> CreateCommand {
    use serenity::all::{CreateCommandOption, CommandOptionType};

    CreateCommand::new("check")
        .description("Check the verification of a user")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "Select the user to check",
            )
            .required(true),
        )
}
