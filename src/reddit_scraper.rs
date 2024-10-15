use roux::{self, Subreddit};

#[derive(Debug)]
pub struct Post {
    id: String,
    title: String,
    content: String,
    permalink: String,
    subreddit: String,
    author: String,
    over_18: bool,
    num_comments: u64,
    score: f64,
    ups: f64,
    downs: f64,
    created: f64,
    comments: Vec<Comment>,
}

impl Post {
    fn new(
        id: String,
        title: String,
        content: String,
        permalink: String,
        subreddit: String,
        author: String,
        over_18: bool,
        num_comments: u64,
        score: f64,
        ups: f64,
        downs: f64,
        created: f64
    ) -> Post {
        Post {
            id,
            title,
            content,
            permalink,
            subreddit,
            author,
            over_18,
            num_comments,
            score,
            ups,
            downs,
            created,
            comments: Vec::new()
        }
    }

    fn set_comments(self, comments: Vec<Comment>) -> Post {
        Post {
            id: self.id,
            title: self.title,
            content: self.content,
            permalink: self.permalink,
            subreddit: self.subreddit,
            author: self.author,
            over_18: self.over_18,
            num_comments: self.num_comments,
            score: self.score,
            ups: self.ups,
            downs: self.downs,
            created: self.created,
            comments
        }
    }
}

#[derive(Debug)]
pub struct Comment {
    id: String,
    post_id: String,
    parent_id: Option<String>,
    author: Option<String>,
    permalink: Option<String>,
    body_html: Option<String>,
    over_18: Option<bool>,
    score: Option<i32>,
    ups: Option<i32>,
    downs: Option<i32>,
}

pub async fn scrape_subreddit(name: &str) -> Vec<Post> {
    let subreddit = Subreddit::new(name);
    let latest = subreddit.latest(100, None).await;

    let posts = match latest {
        Ok(posts) => {
            posts.data.children.iter().filter_map(|post| {
                let post = &post.data;
                let content = match &post.selftext_html {
                    Some(content) => content,
                    None => match &post.url {
                        Some(content) => content,
                        None => return None
                    }
                };

                Some(Post::new(
                    post.id.clone(),
                    post.title.clone(),
                    content.clone(),
                    post.permalink.clone(),
                    post.subreddit.clone(),
                    post.author.clone(),
                    post.over_18,
                    post.num_comments,
                    post.score,
                    post.ups,
                    post.downs,
                    post.created
                ))
            }).collect()
        },
        Err(_) => Vec::new(),
    };

    let mut posts_with_comments = Vec::new();
    for post in posts {
        let comments = match subreddit.article_comments(
            &post.id,
            Some(post.num_comments.try_into().unwrap_or(100)),
            Some(post.num_comments.try_into().unwrap_or(100))
        ).await {
            Ok(comments) => {
                comments.data.children.iter().filter_map(|comment| {
                    let comment = &comment.data;
                    match &comment.id {
                        Some(id) =>  { Some(Comment {
                            id: id.clone(),
                            post_id: post.id.clone(),
                            parent_id: comment.parent_id.clone(),
                            author: comment.author.clone(),
                            permalink: comment.permalink.clone(),
                            body_html: comment.body_html.clone(),
                            over_18: comment.over_18,
                            score: comment.score,
                            ups: comment.ups,
                            downs: comment.downs,
                        })},
                        None => None
                    }
                }).collect()
            }
            Err(_) => Vec::new(),
        };

        let post = post.set_comments(comments);
        posts_with_comments.push(post);
    }

    posts_with_comments
}
