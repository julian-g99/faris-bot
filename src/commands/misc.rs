use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult, ArgError,
    macros::command,
};

use rand::random;

#[command]
pub fn long_emote(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() || args.remaining() != 2 {
        send_msg!("Invalid number of arguments. Specify one emote and the number.", msg, ctx)?;
        return Ok(());
    }

    let emote = args.single::<String>()?;
    //let emote = match args.single::<String>() {
        //Ok(s) => s,
        //Err(_e) => {
            //send_msg!("Unknown error.", msg, ctx);
            //return Ok(());
        //}
    //};
    let count = match args.single::<u64>() {
        Ok(c) => c,
        Err(e) => {
            match e {
                ArgError::Parse(_e) => {
                    send_msg!("Failed to parse argument. Make sure to specify an integer as the second argument.", msg, ctx)?;
                    return Ok(());
                },
                _ => {
                    send_msg!("Unknown error", msg, ctx)?;
                    return Ok(());
                }
            }
        }
    };

    let mut rtn = String::new();

    for i in 1..count+1 {
        let curr = format!(":{}{}:", emote, i);
        rtn.push_str(curr.as_str());
    }

    send_msg!(rtn.as_str(), msg, ctx)?;
    Ok(())
}

#[command]
pub fn uwuify(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    match args.remains() {
        Some(s) => {
            let result = s.replace(&['l', 'r'][..], "w");
            send_msg!(result, msg, ctx)?;
            return Ok(());
        },
        None => {
            send_msg!("Pwease gib text uwuify~~ <3", msg, ctx)?;
            return Ok(());
        }
    }
}

#[command]
pub fn case(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    match args.remains() {
        Some(s) => {
            let result: String = s.chars().map(|c| {
                let new = if random() {c.to_ascii_lowercase()} else {c.to_ascii_uppercase()};
                new
            }).collect();
            send_msg!(result, msg, ctx)?;
            return Ok(());
        },
        None => {
            send_msg!("Please give text to random case", msg, ctx)?;
            return Ok(());
        }
    }
}

// TODO: finish implementing this
#[command]
pub fn react(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    match send_msg!("React with the trash emoji to delete this message", msg, ctx) {
        Ok(msg) => {
            msg.react(&ctx, "🗑️")?;
        },
        Err(_) => {
            send_msg!("Issue reacting", msg, ctx)?;
        }
    }
    Ok(())
}
