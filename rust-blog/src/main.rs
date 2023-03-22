use axum::{
    http::StatusCode, routing::get, Router,
    response::{Html, IntoResponse},
    extract::{State, Path},
    // extract::{TypedHeader},
    // headers::UserAgent,
};

use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
//use sqlx::PgPool;
use sqlx::FromRow;
use sqlx::types::time::Date;

use askama::Template;

use tower_http::services::ServeDir;

// the fields we'll be retrieving from an sql query
#[derive(FromRow, Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_date: Date,
    pub post_body: String,
}

// Each post template will be populated with the values 
// located in the shared state of the handlers. 
#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostTemplate<'a> {
    pub post_title: &'a str,
    pub post_date: String,
    pub post_body: &'a str,
}

// create an Axum template for our homepage
// index_title is the html page's title 
// index_links are the titles of the blog posts 
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub index_title: String,
    pub index_links: &'a Vec<String>,
}

// Our custom Askama filter to replace spaces with dashes in the title
mod filters {

    // now in our templates we can add this filter e.g. {{ post_title|rmdash }}
    pub fn rmdashes(title: &str) -> askama::Result<String> {
        Ok(title.replace("-", " ").into())
     }
}

// Then populate the template with all post titles
async fn index(State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse{

    let s = state.clone();
    let mut plinks: Vec<String> = Vec::new();

    for i in 0 .. s.len() {
        plinks.push(s[i].post_title.clone());
    }

    let template = IndexTemplate{index_title: String::from("My blog"), index_links: &plinks};

    match template.render() {
            Ok(html) => Html(html).into_response(),
         Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error {}", err),
            ).into_response(),
    }
}

// sample homepage handler
// async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
//     let mut hello = String::from("Welcome! \n");
//     hello.push_str("Your User Agent is: ");
//     hello.push_str(user_agent.as_str());
//     hello
// }

// We use two extractors in the arguments
// Path to grab the query and State that has all our posts 

async fn post(Path(query_title): Path<String>, State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse {

    // A default template or else the compiler complains 
    let mut template = PostTemplate{post_title: "none", post_date: "none".to_string(), post_body: "none"};
    
    // We look for any post with the same title as the user's query
    for i in 0..state.len() {
        if query_title == state[i].post_title {
            // We found one so mutate the template variable and
            // populate it with the post that the user requested 
            template = PostTemplate{post_title: &state[i].post_title, 
                       post_date: state[i].post_date.to_string(), 
                       post_body: &state[i].post_body
            };
            break;
        } else {
            continue
        }
    }

    // 404 if no title found matching the user's query 
    if &template.post_title == &"none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }

    // render the template into HTML and return it to the user
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response()
    }
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
                .max_connections(5)
                // use your own credentials
                .connect("postgres://username:password@localhost/mydb")
                .await
                .expect("couldn't connect to the database");

	// fetch all of the posts at the start of the program 
	// to avoid hitting the db for each page request
    // for dynamic creation of posts, move this to a function to call for every get(index) request
    let mut posts = sqlx::query_as::<_, Post>("select post_title, post_date, post_body from myposts") 
        .fetch_all(&pool)
        .await
        .unwrap();

    for post in &mut posts {
        post.post_title = post.post_title.replace(" ", "-");
        }

	// Above we retrieved Vec<Post> 
	// We place it in an Arc for thread-safe referencing.  
    let shared_state = Arc::new(posts);

    let app = Router::new()
            .route("/", get(index))
            .route("/post/:query_title", get(post))
            .with_state(shared_state)
            .nest_service("/assets", ServeDir::new("assets"));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

