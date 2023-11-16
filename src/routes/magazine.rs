use askama::Template;

#[derive(Template)]
#[template(path = "magazine.html")]
pub struct MagazineTemplate<'a> {
    title: &'a str,
}

pub async fn handler() -> MagazineTemplate<'static> {
    MagazineTemplate {
        title: "Magazine World",
    }
}
