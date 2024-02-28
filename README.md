# Telegram Sticker Bot

This is a simple Telegram bot written in Rust using the `teloxide` library. The bot's main function is to monitor stickers sent in a chat and delete any consecutive stickers sent by the same user. Made in like 15 minutes lmao

## Features

- Monitors stickers sent in a chat.
- Deletes consecutive stickers sent by the same user.
- Logs all actions for easy debugging and tracking.

## Prerequisites

- A Telegram bot token

## Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/telegram-sticker-bot.git
cd telegram-sticker-bot
```

2. Create a `.env` file in the root directory and add your Telegram bot token:

```bash


echo "TELOXIDE_TOKEN=your_bot_token" > .env
```

3. Build and run the bot:

```bash
cargo build
cargo run
```

## Usage

Add the bot to your Telegram group. It will automatically start monitoring for stickers and perform its function.
