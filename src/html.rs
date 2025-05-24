use maud::{html, Markup, DOCTYPE};

fn header(page_title: &str) -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta charset="utf-8";
            link rel="stylesheet" href="static/base.css"
            title { (page_title) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            a href="https://my-search.net" { "my-search.net" }
        }
    }
}

pub fn page(title: &str, page_contents: Markup) -> Markup {
    html! {
        (header(title))
        (page_contents)
        (footer())
    }
}

pub fn not_found() -> Markup {
    page(
        "Not found",
        html! {
            h1 { "404" }
            p { "Not found" }
        },
    )
}
