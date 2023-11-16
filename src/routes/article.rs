use askama::Template;

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate<'a> {
    article_name: &'a str,
}

pub async fn handler() -> ArticleTemplate<'static> {
    let article = ArticleTemplate {
        article_name: "THE BEST ARTICLE EVER"
    };

    article
}
