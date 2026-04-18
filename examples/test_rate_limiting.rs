use fluxer_neptunium::{client::error::Error, model::guild::Guild, prelude::*};
use tokio::{task::JoinSet, time::Instant};
use tracing_subscriber::filter::LevelFilter;

const NUM_REQUESTS: usize = 500;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    let token = std::env::var("FLUXER_TOKEN").unwrap();
    let client = Client::new(token);

    let ctx = client.context();
    let start_time = Instant::now();
    let mut join_set = JoinSet::<Result<Vec<Cached<Guild>>, Error>>::new();
    for i in 1..=NUM_REQUESTS {
        let ctx = ctx.clone();
        join_set.spawn(async move {
            println!("Running #{i}...");
            let result = ctx.list_own_guilds().await;
            println!("#{i} finished!");
            result
        });
    }
    let joined = join_set.join_all().await;
    let end_time = Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;
    for (i, result) in joined.iter().enumerate() {
        if let Err(e) = result {
            // Ideally, should not happen, but may happen when an internal server error or some other
            // error unrelated to rate limiting happens.
            println!("Task #{i} returned error: {e:?} ({e})");
            error_count += 1;
        } else {
            success_count += 1;
        }
    }
    println!(
        "---\nSuccess: {}\nError: {}\nTotal: {}\nTime: {}\n---",
        success_count,
        error_count,
        NUM_REQUESTS,
        end_time.duration_since(start_time).as_secs_f64()
    );
}
