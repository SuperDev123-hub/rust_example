use std::{collections::HashMap, sync::Arc};

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    bodys: Vec<u8>,
}

struct Response {
    code: String,
    headers: HashMap<String, String>,
    bodys: Vec<u8>,
}

type BoxedCalback = Box<dyn Fn(&Request) -> Response>;
struct BasicRouter<C>
where
    C: Fn(&Request) -> Response,
{
    routes: HashMap<String, C>,
}

impl<C> BasicRouter<C>
where
    C: Fn(&Request) -> Response,
{
    fn new() -> Self {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_router(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}
fn main() {
    let abc: Vec<String> = Vec::new();
    let dd = Arc::new(abc);
}
