mod helper_types;

use std::collections::HashMap;

use helper_types::*;
use tower::{Service, ServiceExt};


pub async fn run<App>(mut app: App)
where
    App: Service<Request, Response = Response>,
    App::Error: std::fmt::Debug,
    App::Future: Send + 'static,
     {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            let req = Request {
                path_and_query: "/fake/path?page=1".to_owned(),
                headers: HashMap::new(),
                body: Vec::new(),
            };

            let app = match app.ready().await {
                Err(e) => {
                    eprintln!("Service not able to accept requests: {:?}", e);
                    continue;
                }
                Ok(app) => app,
            };

            let future = app.call(req);
            tokio::spawn(async move {
                match future.await {
                    Ok(res) => println!("Successful response: {:?}", res),
                    Err(e) => eprintln!("Error occurred: {:?}", e),
                }
            });
        }
}

#[tokio::main]
async fn main() {
    run(DemoApp::default()).await;
}
