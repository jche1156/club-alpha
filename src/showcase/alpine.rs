use askama::Template;

#[derive(Template)]
#[template(path = "alpine.html")]
pub struct AlpineTemplate<'a> {
    name: &'a str,
}

pub async fn handler() -> AlpineTemplate<'static> {
    AlpineTemplate { name: "world" }
}
