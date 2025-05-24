use std::error::Error;

use axum::{
    extract::Query,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use html::{not_found, page};
use maud::{html, Markup};
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use rust_embed::Embed;
use serde::Deserialize;
use tracing::info;

mod github;
mod html;

async fn not_found_handler() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, not_found())
}

#[derive(Embed)]
#[folder = "src/static/"]
struct Asset;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("static/") {
        path = path.replace("static/", "");
    }

    StaticFile(path)
}

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, not_found()).into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/static/{*file}", get(static_handler))
        .fallback_service(get(not_found_handler));

    let port = 2772;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();
    info!(port = port, "Listening");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct Search {
    q: Option<String>,
}

async fn root(search: Query<Search>) -> Markup {
    let query = match &search.q {
        None => {
            return page(
                "My Search",
                html! {
                    style {
                        "
                        body {
                            background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
                        }
                        .search-glow {
                            box-shadow: 0 0 20px rgba(239, 68, 68, 0.1);
                        }
                        .search-glow:focus-within {
                            box-shadow: 0 0 30px rgba(239, 68, 68, 0.2);
                        }
                        .search-button:hover {
                            transform: translateY(-1px);
                            box-shadow: 0 8px 25px rgba(30, 64, 175, 0.3);
                        }
                        .fade-in {
                            animation: fadeIn 0.8s ease-out;
                        }
                        @keyframes fadeIn {
                            from {
                                opacity: 0;
                                transform: translateY(20px);
                            }
                            to {
                                opacity: 1;
                                transform: translateY(0);
                            }
                        }
                        .logo-text {
                            background: linear-gradient(135deg, #dc2626, #1e40af);
                            -webkit-background-clip: text;
                            -webkit-text-fill-color: transparent;
                            background-clip: text;
                        }
                        "
                    }
                    div class="min-h-screen flex items-center justify-center p-4" {
                        div class="w-full max-w-2xl mx-auto fade-in" {
                            div class="text-center mb-12" {
                                h1 class="text-6xl font-bold logo-text mb-2" {
                                    "My Search"
                                }
                                p class="text-slate-400 text-lg" {
                                    "Find what you're looking for"
                                }
                            }

                            form class="space-y-6" method="get" {
                                div class="relative search-glow rounded-2xl bg-slate-800/50 backdrop-blur-sm border border-slate-700/50 transition-all duration-300" {
                                    input
                                        type="text"
                                        name="q"
                                        placeholder="Enter your search term..."
                                        class="w-full px-6 py-4 text-lg bg-transparent text-white placeholder-slate-400 focus:outline-none rounded-2xl"
                                        autocomplete="off"
                                        required;
                                    div class="absolute right-4 top-1/2 transform -translate-y-1/2" {
                                        svg class="w-6 h-6 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z";
                                        }
                                    }
                                }

                                div class="text-center" {
                                    button
                                        type="submit"
                                        class="search-button inline-flex items-center px-8 py-3 bg-blue-800 hover:bg-blue-700 text-white font-semibold rounded-xl transition-all duration-300 focus:outline-none focus:ring-4 focus:ring-blue-800/30" {
                                        svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z";
                                        }
                                        "Search"
                                    }
                                }
                            }
                        }
                    }
                    script {
                        "
                        // Focus on search input when page loads
                        document.addEventListener('DOMContentLoaded', function () {
                            const searchInput = document.querySelector('input[name=\"q\"]');
                            if (searchInput) {
                                searchInput.focus();
                            }
                        });
                        "
                    }
                },
            );
        }
        Some(q) => q,
    };

    let gh_resp = get_github(query.clone()).await;

    let res = match gh_resp {
        Err(e) => {
            println!("{e:?}");
            return page(
                "Search Error",
                html! {
                    style {
                        "
                        body {
                            background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
                        }
                        .error-card {
                            background: rgba(30, 41, 59, 0.5);
                            backdrop-filter: blur(12px);
                            border: 1px solid rgba(71, 85, 105, 0.5);
                        }
                        .error-glow {
                            box-shadow: 0 0 20px rgba(239, 68, 68, 0.1);
                        }
                        .fade-in {
                            animation: fadeIn 0.8s ease-out;
                        }
                        @keyframes fadeIn {
                            from {
                                opacity: 0;
                                transform: translateY(20px);
                            }
                            to {
                                opacity: 1;
                                transform: translateY(0);
                            }
                        }
                        .logo-text {
                            background: linear-gradient(135deg, #dc2626, #1e40af);
                            -webkit-background-clip: text;
                            -webkit-text-fill-color: transparent;
                            background-clip: text;
                        }
                        .try-again-button:hover {
                            transform: translateY(-1px);
                            box-shadow: 0 8px 25px rgba(30, 64, 175, 0.3);
                        }
                        "
                    }
                    div class="min-h-screen flex items-center justify-center p-4" {
                        div class="w-full max-w-md fade-in" {
                            div class="error-card error-glow rounded-2xl p-8 transition-all duration-300" {
                                h1 class="text-2xl font-bold text-center mb-4 logo-text" {
                                    "Search Error"
                                }
                                p class="text-center text-slate-300 mb-6" {
                                    (e.to_string())
                                }
                                a href="/" class="try-again-button block w-full py-3 bg-blue-800 hover:bg-blue-700 text-white text-lg font-semibold rounded-xl text-center transition-all duration-300 focus:outline-none focus:ring-4 focus:ring-blue-800/30" {
                                    "Try Again"
                                }
                            }
                        }
                    }
                },
            );
        }
        Ok(v) => v,
    };

    page(
        "Search Results",
        html! {
            style {
                "
                body {
                    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
                }
                .search-card, .results-card {
                    background: rgba(30, 41, 59, 0.5);
                    backdrop-filter: blur(12px);
                    border: 1px solid rgba(71, 85, 105, 0.5);
                }
                .search-glow {
                    box-shadow: 0 0 20px rgba(239, 68, 68, 0.1);
                }
                .search-glow:focus-within {
                    box-shadow: 0 0 30px rgba(239, 68, 68, 0.2);
                }
                .search-button:hover {
                    transform: translateY(-1px);
                    box-shadow: 0 8px 25px rgba(30, 64, 175, 0.3);
                }
                .result-item {
                    background: rgba(51, 65, 85, 0.3);
                    border: 1px solid rgba(71, 85, 105, 0.4);
                    transition: all 0.3s ease;
                }
                .result-item:hover {
                    background: rgba(51, 65, 85, 0.5);
                    border-color: rgba(71, 85, 105, 0.6);
                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
                }
                .fade-in {
                    animation: fadeIn 0.8s ease-out;
                }
                @keyframes fadeIn {
                    from {
                        opacity: 0;
                        transform: translateY(20px);
                    }
                    to {
                        opacity: 1;
                        transform: translateY(0);
                    }
                }
                "
            }
            div class="min-h-screen p-4" {
                div class="max-w-4xl mx-auto" {
                    // Header with search form
                    div class="search-card search-glow rounded-2xl p-6 mb-6 transition-all duration-300 fade-in" {
                        form class="flex gap-4" method="get" {
                            div class="flex-1 relative" {
                                input
                                    type="text"
                                    name="q"
                                    value=(query)
                                    placeholder="Enter search term..."
                                    class="w-full px-4 py-3 text-lg bg-slate-800/50 text-white placeholder-slate-400 border border-slate-700/50 rounded-xl focus:outline-none focus:border-red-500/50 focus:ring-2 focus:ring-red-500/20 transition-all duration-200";
                            }
                            button
                                type="submit"
                                class="search-button px-6 py-3 bg-blue-800 hover:bg-blue-700 text-white text-lg font-semibold rounded-xl transition-all duration-300 focus:outline-none focus:ring-4 focus:ring-blue-800/30" {
                                "Search"
                            }
                        }
                    }

                    // Results
                    div class="results-card rounded-2xl p-6 transition-all duration-300 fade-in" {
                        h2 class="text-2xl font-bold mb-4 text-white" {
                            "Results (" (res.total_count) ")"
                        }

                        @if res.items.is_empty() {
                            p class="text-center text-slate-400 py-8" {
                                "No results found for \"" (query) "\""
                            }
                        } @else {
                            div class="space-y-4" {
                                @for item in res.items {
                                    div class="result-item rounded-xl p-4" {
                                        h3 class="text-lg font-semibold text-white mb-2" {
                                            (item.title)
                                        }
                                        a href=(item.html_url) target="_blank" class="text-blue-400 hover:text-blue-300 transition-colors duration-200 text-sm break-all" {
                                            (item.html_url)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}

async fn get_github(q: String) -> Result<github::Root, Box<dyn Error>> {
    // https://api.stackexchange.com/2.3/search/advanced?order=desc&sort=activity&site=stackoverflow&q=js%20fibonacci
    // https://api.github.com/search/issues?q=box%20error
    let get_resp = reqwest::Client::new()
        .get(format!("https://api.github.com/search/issues?q={q}"))
        .header(USER_AGENT, "my-search")
        .send()
        .await?;

    if let Some(ref ct) = get_resp.headers().get(CONTENT_TYPE) {
        if !ct.to_str()?.starts_with("application/json") {
            println!("content-type {:?}", ct.to_str());
            let txt = get_resp.text().await?;
            println!("resp text : {txt}");
            return Err("error".into());
        }
    }
    println!("get_resp: {get_resp:?}");
    let resp = get_resp.json::<github::Root>().await?;
    println!("resp: {resp:?}");
    Ok(resp)
}
