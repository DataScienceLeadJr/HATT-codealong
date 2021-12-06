use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use std::{collections::HashMap, sync::atomic::AtomicUsize};
use std::sync::Arc;


pub struct Request {
    pub path_and_query: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct Response {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Default)]
pub struct DemoApp {
    counter: Arc<AtomicUsize>,
}

impl tower::Service<Request> for DemoApp {
    type Response = Response;
    type Error = anyhow::Error;

    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let counter = self.counter.clone();
        Box::pin(async move {
            println!("Handling a request for {}", req.path_and_query);
            let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            anyhow::ensure!(counter % 4 != 2, "Don't mind me! Just failing 25% of the time, just for fun!");
            req.headers
                .insert("X-Counter".to_owned(), counter.to_string());
            let res = Response {
                status: 200,
                headers: req.headers,
                body: req.body,
            };
            Ok::<_, anyhow::Error>(res)
        })
    }
}
