use std::sync::Arc;

use serenity::{
    all::{CommandInteraction, Http},
    builder::CreateCommand,
};

use crate::structs::VerifiedUser;

pub async fn run(http: &Arc<Http>, interaction: &CommandInteraction) -> String {
    crate::check_role_available!(interaction.user.id.get());

    let coll = crate::mongo::get_coll::<VerifiedUser>("verified_users");
    let query = doc! {
        "id": interaction.user.id.to_string(),
    };
    let _ = coll.delete_many(query).await;

    let settings = crate::settings();
    let guild_id = settings.get_guild().get_id().into();
    let role_id = settings.get_guild().get_verify_role_id().into();
    let _ = http.remove_member_role(guild_id, interaction.user.id, role_id, None).await;

    format!(
        "Hello <@{}> :)\n\nAll your data has been deleted and roles removed!",
        interaction.user.id
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reset")
        .description("Delete all your data from our database and remove all roles")
}
