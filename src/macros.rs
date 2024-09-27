#[macro_export]
macro_rules! check_role_available {
    () => {
        if $crate::settings().get_guild().get_verify_role_id() < $crate::LOWEST_ID {
            return format!(
                "No role has been set for verified users! Use the command `/init`!",
            );
        }
    };

    ($user_id:expr) => {
        if $crate::settings().get_guild().get_verify_role_id() < $crate::LOWEST_ID {
            return format!(
                "Hello <@{}> :)\n\nNo role has been set for verified users! Use the command `/init`!",
                $user_id
            );
        }
    };

    ($http:expr, $user_id:expr) => {
        $crate::check_role_available!($user_id);

        let settings = $crate::settings();
        let (role_id, guild_id) = (settings.get_guild().get_verify_role_id(), settings.get_guild().get_id());
        let roles = $http.get_guild_roles(guild_id.into()).await.unwrap();
        if !roles.iter().any(|role| role.id.get() == role_id) {
            return format!(
                "Hello <@{}> :)\n\nThe role with the ID `{}` does not exist! Please remove `verify_role_id` from the `settings.json` file, restart the bot and execute the command `/init` again!",
                $user_id,
                role_id
            );
        }
    };
}