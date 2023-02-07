use async_openai::{
    types::{CreateImageRequestArgs, ImageData, ImageSize, ResponseFormat},
    Client,
};
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

use url::Url;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Command::repl(bot, command_handler).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Telegram Bot Openai.")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start bot.")]
    Start,
    #[command(description = "generate image from text")]
    Prompt(String),
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Hello, I'm Nightly Bot, I was made to create images from text using the openai library. usage: /prompt <text>").await;
        }
        Command::Prompt(args) => {
            if args == "" {
                bot.send_message(msg.chat.id, "Description?").await;
                return Ok(());
            }
            bot.send_message(msg.chat.id, "Wait").await;

            let request = CreateImageRequestArgs::default()
                .prompt(args)
                .n(1)
                .response_format(ResponseFormat::Url)
                .size(ImageSize::S1024x1024)
                .user("async-openai")
                .build()
                .unwrap();

            let response = Client::new().images().create(request).await.unwrap();
            if let ImageData::Url(url) = response.data[0].as_ref() {
                bot.send_photo(
                    msg.chat.id,
                    InputFile::url(Url::parse(url.as_str()).unwrap()),
                )
                .await;
            }
        }
    };
    Ok(())
}
