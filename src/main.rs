use egg_mode::tweet::DraftTweet;
use rand::prelude::*;
extern crate cronjob;
use cronjob::CronJob;
mod config;
mod tweets;

#[tokio::main]
async fn main() /* -> Result<(), Box<dyn std::error::Error>>  */
{
    // create a new cron job handler
    let hours = "16"; // 10:00AM;
    let seconds = "0";
    let minutes = "45";
    let mut cron_job = CronJob::new("Update Twitter Status", handle_task);
    cron_job.offset(1);
    cron_job.seconds(seconds);
    cron_job.minutes(minutes);
    cron_job.hours(hours);
    cron_job.start_job();
    let result = dispatch_tweets().await.ok();
    println!("success {} ", Some(result).is_none())
}

async fn dispatch_tweets() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hey I'm CRON {}", argz);
    /*
    get the size of the tweet index array,
     to be used to constrain range of random numbers to  be generated
    in the following section
    */
    let draft_tweet_size: &usize = &tweets::tweets().len();
    println!("{}", draft_tweet_size);
    //generate a random number to be used to fetch a saved tweet
    let mut random_number = thread_rng();
    let random_tweet_index = random_number.gen_range(0..*draft_tweet_size);
    let draft_tweet = &tweets::tweets()[random_tweet_index];

    let tweet = format!(
        r#"
              "{quote}"
              
-{quoter}

quote from #raven 
#bot #Rust #Rustlang 
    "#,
        quote = draft_tweet.quote,
        quoter = draft_tweet.quoter
    );

    let tweet = DraftTweet::new(tweet);
    println!("{:?}", tweet);
    // Twitter SDK configuration
    let config = config::Config::load().await;
    tweet.send(&config.token).await?;
    // https://github.com/opeolluwa/raven

    Ok(())
}

fn handle_task(argz: &str) {
    // plug in dispatch tweets here
    println!("hey {:?}", argz);
}
