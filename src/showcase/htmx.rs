use askama::Template;

#[derive(Template)]
#[template(path = "htmx.html")]
pub struct HtmxReadTemplate {}

#[derive(Template)]
#[template(path = "htmx-edit.html")]
pub struct HtmxEditTemplate {}

pub async fn read() -> HtmxReadTemplate {
    HtmxReadTemplate {}
}

pub async fn edit() -> HtmxEditTemplate {
    HtmxEditTemplate {}
}
