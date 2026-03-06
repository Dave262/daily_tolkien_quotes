# Daily JRR Tolkien Quotes to a Telegram bot

This is a simple script written in Rust that randomly pulls a quote from a database of JRR Tolkien quotes (quotes.json), formats them, and send them to a Telegram bot. I then run it as a systemd service on my home server. 

<img src="./chat_example_pic.png" alt="example telegram chat image" width="400" style="border-radius: 10px;">
I got the quotes from the very fabulous http://lotrproject.com/


## Make it work 

If you clone the repo you just need to set up a Telegram bot and get the BOT_TOKEN
[set up a telegram bot tutorial](https://core.telegram.org/bots/tutorial)
Then get the CHAT_ID - instructions for how to do that are here https://stackoverflow.com/questions/32423837/telegram-bot-how-to-get-a-group-chat-id

create an .env file in the project root directory and add the BOT_TOKEN and CHAT_ID to it like this - note the bot token is taken as a string and the chat id is an integer with no quotes. 

```bash
TELEGRAM_BOT_TOKEN="<bot token string>"s
CHAT_ID=<chat  id integer>
```

Then build the binary 

```bash
cargo build --release
```

and copy resulting file (generally located at `target/release/tolkien_quotes`) to your bin folder. also make sure you make it executable. For me it was by default but worth checking. 

To run it daily (or however often you want) set up a systemd timer. Something like this: 

create `/etc/systemd/system/daily-tolkien-quote.service
with something like this 

```
[Unit]
Description=daily tolkien quote service
After=network-online.target
Wants=network-online.target

# Replace items in < > with your own details 
[Service]
Type=oneshot
User=<whatever user you want to run the script as>
WorkingDirectory=<i had this asd my home folder /home/user>
ExecStart=<path to the executable>

Environment=RUST_BACKTRACE=1

[Install]
WantedBy=multi-user.target
```

and `/etc/systemd/system/daily-tolkien-quote.timer`

```
[Unit]
Description=Run tolkien_quotes at 11:11 ever day - sends them to telegarm bot.

[Timer]
OnCalendar=*-*-* 11:11:00
Persistent=true

[Install]
WantedBy=timers.target
```

enable it all with `systemctl enable <name of service>`
