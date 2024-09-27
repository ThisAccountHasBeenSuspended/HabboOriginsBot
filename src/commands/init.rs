use serenity::{
    all::{CommandDataOptionValue, CommandInteraction},
    builder::CreateCommand,
};

pub async fn run(interaction: &CommandInteraction) -> String {
    if interaction.data.options.is_empty() {
        return format!(
            "Hello <@{}> :)\n\nThe role is missing!",
            interaction.user.id,
        );
    }

    let member = interaction.member.as_ref().unwrap();
    let permissions = member.permissions.unwrap();
    if !permissions.administrator() {
        return format!(
            "Hello <@{}> :)\n\nYou are not allowed to execute this command!",
            interaction.user.id
        );
    }

    if crate::settings().get_guild().get_verify_role_id() >= crate::LOWEST_ID {
        return format!(
            "Hello <@{}> :)\n\nThis bot has already been initialized!",
            interaction.user.id
        );
    }

    let role_id = match &interaction.data.options.first().unwrap().value {
        CommandDataOptionValue::Role(val) => val,
        _ => {
            return format!(
                "Hello <@{}> :)\n\nNot a valid role!",
                interaction.user.id,
            );
        }
    };

    unsafe {
        crate::settings().as_mut().get_guild_mut().set_verify_role_id(role_id.get());
    };
    crate::settings().save();

    format!(
        "Hello <@{}> :)\n\nRole selected: <@&{}>",
        interaction.user.id,
        role_id.get()
    )
}

pub fn register() -> CreateCommand {
    use serenity::all::{CreateCommandOption, CommandOptionType};

    CreateCommand::new("init")
        .description("Initialize this bot")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Role,
                "role",
                "Select or create a role",
            )
            .required(false),
        )
}
