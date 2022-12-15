use egg_mode::tweet::DraftTweet;
use rand::prelude::*;
extern crate cronjob;
use cronjob::CronJob;
mod config;
mod tweets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new cron job handler
    let hours = "18"; // 10:00AM;
    let seconds = "0,2,4,36,42,58";
    let minutes = "55";
    let mut cron_job = CronJob::new("Update Twitter Status", handler);
cron_job.offset(0);
    // cron_job.day_of_week("Mon,Tue,Wed,Thur,Fri");
    // cron_job.hours(hours);
    cron_job.minutes(minutes);
    cron_job.seconds(seconds);
    cron_job.start_job();

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

    "#,
        quote = draft_tweet.quote,
        quoter = draft_tweet.quoter
    );
    // println!("{:?}", tweet);

    let tweet = DraftTweet::new(tweet);

    // Twitter SDK configuration
    let config = config::Config::load().await;
    // tweet.send(&config.token).await?;
    Ok(())
}

fn handler(argz: &str) {
    println!("hey im cronn {}", argz);
}
