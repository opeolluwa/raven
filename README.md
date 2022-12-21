<img src="./raven.png" style="display:block;margin-bottom:5rem; object-fit:contain;"/>

# Raven
A tweet scheduler 

## What is it?
Raven is a bot that updates my Twitter status on my behalf each day  

## How it works
Raven retrieves predefined tweets, and dispatch then when the scheduled time is right

## How to use it
1. Clone the repo
2. Create a `.env` file and add your Twitter API keys
3. Add your tweets to the `tweets.json` file
4. Run `cargo run` to install dependencies
5. Run `cargo build --release` for production build

## Requirement
The following are required to run the application in development or in production environment
- [Rust](https://www.rust-lang.org/tools/install) v1.63 or greater 
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change,
