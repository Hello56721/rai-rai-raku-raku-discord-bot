use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;

use std::process::Command;

use super::OWNER_ID;

pub async fn restart(context: Context, command: ApplicationCommandInteraction) {
    let (should_shut_down, response_content) = if command.user.id != OWNER_ID {
        (
            false,
            "sorry but i dont take orders from idiots like u".to_string(),
        )
    } else if let Err(why) = Command::new("cargo").arg("run").spawn() {
        (false, format!("failed to restart bot bc of {:?}", why))
    } else {
        (true, "restarting bot".to_string())
    };

    command
        .create_interaction_response(context, |response| {
            response
                .interaction_response_data(|message| {
                    if command.user.id == OWNER_ID {
                        message.content(response_content).ephemeral(true)
                    } else {
                        message.content(response_content).ephemeral(false) // for completeness
                    }
                })
                .kind(InteractionResponseType::ChannelMessageWithSource)
        })
        .await
        .expect("bozo failure");

    if should_shut_down {
        std::process::exit(0);
    }
}

pub async fn dm(context: Context, command: ApplicationCommandInteraction) {
    let user = command
        .data
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();
    let message = command
        .data
        .options
        .get(1)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::User(user, _member) = user {
        if let CommandDataOptionValue::String(message) = message {
            let response = async {
                let dm_channel = user.create_dm_channel(context.clone()).await;

                if let Err(_) = dm_channel {
                    format!("cant dm {} :pouting_cat:", user.name)
                } else if let Ok(dm_channel) = dm_channel {
                    let result = dm_channel
                        .send_message(context.clone(), |msg| msg.content(message))
                        .await;

                    if let Err(_) = result {
                        format!("cant dm {} :pouting_cat:", user.name)
                    } else {
                        "done :can_pooper:".to_string()
                    }
                } else {
                    String::new()
                }
            }
            .await;

            let result = command
                .create_interaction_response(context, |response_data| {
                    response_data
                        .interaction_response_data(|response_data| response_data.content(response))
                })
                .await;

            if let Err(error) = result {
                eprintln!("[ERROR]: Failed to respond to DM command: {:?}", error);
            }
        }
    }
}
