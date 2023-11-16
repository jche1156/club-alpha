use axum::extract::Path;
use maud::{html, Markup};

pub async fn handler(Path(user_name): Path<String>) -> Markup {
    html! {
        p { "Greetings, " (user_name) "!" }
    }
}
