use tokio;
use serde_json;
use reqwest::Client;

mod reddit_scraper;

// subreddits to scrape
const SUBREDDITS: [&str; 4] = ["Transgender_Surgeries", "MtF", "trans", "ftm"];

#[tokio::main]
async fn main() {
    let client = Client::new();

    for subreddit in SUBREDDITS {
        let posts = reddit_scraper::scrape_subreddit(subreddit).await;
        let posts = serde_json::to_string(&posts).unwrap();

        client
            .post("https://big-data-course-project-3f321868afd2.herokuapp.com/posts")
            .body(posts)
            .send()
            .await
            .unwrap();
    }
}
