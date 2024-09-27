use std::sync::Arc;

use serenity::all::{CommandInteraction, Http};

#[deny(clippy::mut_from_ref)]
pub unsafe fn ref_to_refmut<T>(val: &T) -> &mut T {
    (val as *const T as *mut T).as_mut().unwrap_unchecked()
}

pub async fn reqwest<T, R>(url: &str, cb: T) -> (bool, Option<R>)
where
    T: Fn(reqwest::Response) -> R,
{
    let client = reqwest::Client::new();
    let req = client
        .get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/1.22 (compatible; MSIE 5.01; PalmOS 3.0) EudoraWeb 2",
        );

    let res = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return (false, None);
        }
    };

    (true, Some(cb(res)))
}

pub async fn edit_reply(http: &Arc<Http>, msg: String, interaction: &CommandInteraction) {
    use serenity::all::EditInteractionResponse;

    if let Err(e) = interaction.edit_response(http, EditInteractionResponse::new().content(msg)).await {
        error!("Cannot edit respond: {}", e);
    }
}

pub async fn reply(http: &Arc<Http>, msg: String, interaction: &CommandInteraction) {
    use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};

    let data = CreateInteractionResponseMessage::new()
        .content(msg)
        .ephemeral(true);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(e) = interaction.create_response(http, builder).await {
        error!("Cannot create respond: {}", e);
    }
}
