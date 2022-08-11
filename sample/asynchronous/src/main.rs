use failure::Fail;
use futures::executor::block_on;
use reqwest::{Client, Url};
use std::thread;
use std::thread::ThreadId;
use std::time::{Duration, SystemTime};

pub type ResultWithError<T> = std::result::Result<T, ErrorWrapper>;

#[derive(Fail, Debug)]
pub enum ErrorWrapper {
    #[fail(display = "http request error: {:?}", error)]
    HttpRequestError { error: reqwest::Error },
}

impl From<reqwest::Error> for ErrorWrapper {
    fn from(error: reqwest::Error) -> Self {
        ErrorWrapper::HttpRequestError { error }
    }
}

fn main() {
    block_on(request_with_async_await());
}

async fn request_with_async_await() {
    output1("START ASYNC AWAIT");
    let start_at = SystemTime::now();

    let client = Client::new();
    let future1 = send_request_async(&client);
    let future2 = send_request_async(&client);
    match future1.await {
        Ok((thread_id, text)) => output2(thread_id, text.as_str()),
        Err(error) => output1(format!("{:?}", error).as_str()),
    };
    match future2.await {
        Ok((thread_id, text)) => output2(thread_id, text.as_str()),
        Err(error) => output1(format!("{:?}", error).as_str()),
    };

    let spent_time = start_at.elapsed().unwrap_or(Duration::new(0, 0));
    output1(format!("END: {}", spent_time.as_millis()).as_str())
}

async fn send_request_async(client: &Client) -> ResultWithError<(ThreadId, String)> {
    let mut response = client
        .get(Url::parse("http://localhost:9000/timestamp").unwrap())
        .send()?;
    Ok((thread::current().id(), response.text()?))
}

fn output2(thread_id: ThreadId, text: &str) {
    println!("[{:?}] => {}", thread_id, text);
}

fn output1(text: &str) {
    output2(thread::current().id(), text);
}