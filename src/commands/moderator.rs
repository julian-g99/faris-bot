use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult, ArgError,
    macros::command,
};

#[command]
pub fn delete(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.remaining() != 1 || args.is_empty() {
        let _ = msg.channel_id.say(&ctx.http, "Invalid number of arguments. Should have exactly one integer argument for the number of messages to delete");
        Ok(())
    } else {
        let count = match args.single::<u64>() {
            Ok(c) => c,
            Err(e) => {
                match e {
                    ArgError::Parse(_e) => {
                        let _ = msg.channel_id.say(&ctx.http, "Failed to parse argument. Make sure to specify an integer for number of messsages to delete.");
                        return Ok(());
                    },
                    _ => {
                        let _ = msg.channel_id.say(&ctx.http, "Unknown error.");
                        return Ok(());
                    }
                }
            }
        };
        //let messages: Vec<MessageId> = msg.channel_id.messages(&ctx.http, |retriever| {
            //retriever.before(msg.id).limit(count)
        //})?.into_iter().map(|m| m.id).collect();
        let messages: Vec<Message> = msg.channel_id.messages(&ctx.http, |retriever| {
            retriever.before(msg.id).limit(count)
        })?;
        msg.channel_id.delete_message(&ctx.http, msg)?;
        msg.channel_id.delete_messages(&ctx.http, messages)?;
        Ok(())
    }
}
