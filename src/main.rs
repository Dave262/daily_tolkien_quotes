mod send_quote;
use anyhow::Result;
use tokio;
use send_quote::Quote;
use dotenvy::dotenv;



#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let quote = &load_quotes();

    let rand_quote = send_quote::get_rand_quote(quote);

    match rand_quote {
        Some(q) => {
            send_quote::send_quote(q.to_owned()).await?;
        },
        None =>  {
            println!("None")
        }
    }
    Ok(())
}

fn load_quotes() -> Vec<Quote> {
    let data = include_str!("../quotes.json");
    serde_json::from_str(&data).expect("Could not parse quotes.json")
}
