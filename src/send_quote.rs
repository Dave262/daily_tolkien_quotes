use std::time::SystemTime;
use anyhow::Result;
use teloxide::{prelude::*, types::ParseMode};
use chrono::DateTime;
use chrono::offset::Local;
use serde::{Deserialize, Serialize};
use std::env;
// use crate::scraper::Quote;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
   pub  text: String,
   pub character: Option<String>,
   pub chapter: Option<String>,
   pub book: Option<String>,
}

pub fn get_rand_quote(quotes: &[Quote]) -> Option<&Quote> {

    if quotes.is_empty() {return None;}
    let rng = rand::random_range(0..quotes.len());

    quotes.get(rng)

}

pub async fn send_quote(quote: Quote) -> Result<()> {

    let date = get_system_date();

    let quote_text = format!(
        "<b>Daily Tolkien Quote</b> - <i>{}</i>\n\n<i>{}</i> ~ {}\n\nFrom '{} - <i>{}</i>'",
        date,
        quote.text,
        quote.character.unwrap_or_else(|| "J.R.R. Tolkien".to_string()),
        quote.book.unwrap_or_else(|| "Book unknown".to_string()),
        quote.chapter.unwrap_or_else(|| "Unknown Chapter".to_string())
    );
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("CHAT_ID must be set").parse().expect("CHAT_ID must be a valid integer");

    let bot = Bot::new(bot_token);
    bot.send_message(ChatId(chat_id), quote_text ).parse_mode(ParseMode::Html).await?;

Ok(())
}

fn get_system_date() -> String{
    let sys_time = SystemTime::now();
    let date_time: DateTime<Local>  = sys_time.into();
   date_time.format("%a %h %Y - %I:%M %p").to_string()
}
