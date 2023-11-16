use axum::extract::Path;
use maud::{html, Markup};

pub async fn handler(Path(user_name): Path<String>) -> Markup {
    html! {
        a href="/" { "Home" }
        p {
            "Greetings, stranger. It appears that you came seekings..."
            br; br;
            (user_name)
        }
        p { "All I have is a 404 error. Take this and be on your way." }
    }
}
