use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult, ArgError,
    macros::command,
};

#[command]
pub fn delete(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // checking permission
    match msg.member(&ctx.cache) {
        None => {
            send_msg!("Failed to get author",msg, ctx);
            return Ok(());
        },
        Some(user) => {
            if !user.permissions(&ctx.cache)?.manage_messages() {
                send_msg!("You must have manage messages permission to use this command", msg, ctx);
                return Ok(());
            }
        }
    }


    if args.remaining() != 1 || args.is_empty() {
        send_msg!("Invalid number of arguments. Should have exactly one integeer argument for number of messages to delete", msg, ctx);
        Ok(())
    } else {
        let count = match args.single::<u64>() {
            Ok(c) => c,
            Err(e) => {
                match e {
                    ArgError::Parse(_e) => {
                        send_msg!("Failed to parse argument. Make sure to specify an integer for number of messages to delete", msg, ctx);
                        return Ok(());
                    },
                    _ => {
                        send_msg!("Unknown error", msg, ctx);
                        return Ok(());
                    }
                }
            }
        };
        let messages: Vec<Message> = msg.channel_id.messages(&ctx.http, |retriever| {
            retriever.before(msg.id).limit(count)
        })?;
        msg.channel_id.delete_message(&ctx.http, msg)?;
        msg.channel_id.delete_messages(&ctx.http, messages)?;
        Ok(())
    }
}
