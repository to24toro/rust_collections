use momento::momento::auth;
use quanta::Clock;
use momento::response::cache_get_response::MomentoGetStatus;
use momento::simple_cache_client::SimpleCacheClientBuilder;
use std::env;
use std::process;
use std::num::NonZeroU64;

async fn f() {
    let clock = Clock::new();
    const N:u32 = 1_000_000;
    let start = clock.now();
    let mut stop = start;
    let auth_token = match env::var("MOMENT_TOKEN") {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, "MOMENT_TOKEN");
            process::exit(1);
        },
    };
    let item_default_ttl_seconds = 60;
    let mut cache_client = match SimpleCacheClientBuilder::new(
        auth_token,
        NonZeroU64::new(item_default_ttl_seconds).unwrap(),
    ) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{}", err);
            panic!("create cache_client error")
        }
    }
    .build();

    let cache_name = String::from("cache");
    match cache_client.create_cache(&cache_name).await {
        Ok(_) => {
            println!("create_cache OK!")
        }
        Err(err) => {
            eprintln!("{}", err);
            panic!("create_cache error")
        }
    }

    let key = String::from("my_key");
    let value = String::from("my_value");
    println!("Setting key: {}, value: {}", key, value);
    match cache_client.set(&cache_name, key.clone(), value.clone(), None).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
        }
    };

    match cache_client.get(&cache_name, key.clone()).await {
        Ok(r) => match r.result {
            MomentoGetStatus::HIT => println!("cache hit!"),
            MomentoGetStatus::MISS => println!("cache miss"),
            _ => println!("error occurred"),
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    };

    match cache_client.delete_cache(&cache_name).await {
        Ok(_) => {
            println!("Permanently deleted cache named, {}", cache_name);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    };

    stop = clock.now();
    println!("quanta::clock::now() overhead = {:?}",stop.duration_since(start));

}


fn main() {
    f();
}
