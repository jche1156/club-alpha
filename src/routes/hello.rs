use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn handler() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}
