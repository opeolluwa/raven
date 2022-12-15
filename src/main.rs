use egg_mode::tweet::DraftTweet;
use rand::prelude::*;

mod config;
mod tweets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    get the size of the tweet index array,
     to be used to constrain range of random numbers to  be generated
    in the following section
    */
    let draft_tweet_size: &usize = &tweets::tweets().len();

    //generate a random number to be used to fetch a saved tweet
    let mut random_number = thread_rng();
    let random_tweet_index = random_number.gen_range(0..*draft_tweet_size);
    let draft_tweet = &tweets::tweets()[random_tweet_index];

    let tweet = format!(
        r#"
              "{quote}"
              
            -{quoter}

quote from #raven 
#bot #rust 
https://github.com/opeolluwa/raven
    "#,
        quote = draft_tweet.quote,
        quoter = draft_tweet.quoter
    );
    println!("{:?}", tweet);

    let tweet = DraftTweet::new(tweet);

    // Twitter SDK configuration
    let config = config::Config::load().await;
    tweet.send(&config.token).await?;
    Ok(())
}
