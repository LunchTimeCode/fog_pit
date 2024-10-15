use askama::Template;
use axum::response::Html;

#[derive(Debug, Template)]
#[template(path = "page.html")]
struct PageTemplate {
    title: String,
}

pub async fn render_page_html() -> Html<String> {
    let page = PageTemplate {
        title: "Fog Pit".to_string(),
    };
    Html(page.render().unwrap())
}

#[derive(Debug, Template)]
#[template(path = "content.html")]
struct ContentTemplate;

pub async fn render_content_html() -> Html<String> {
    let content = ContentTemplate;
    Html(content.render().unwrap())
}
