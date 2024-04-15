use maud::{Render, html, DOCTYPE};
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::AddData,
    web::{Data, Html, Query},
    EndpointExt, Route, Server,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct TextSearch {
    search: String,
    take: Option<usize>,
}

#[handler]
fn index() -> Html<String> {
    Html(
        html! { (DOCTYPE) html {
            head {
                script src="https://unpkg.com/htmx.org@1.9.11"{}
                style { """
table {
	min-width: 90%;
	margin-left: 5%;
	margin-right: 5%;
	margin-bottom: 5px;
	border-collapse: collapse;
	letter-spacing: 1px;
	font-size: 0.8rem;

	td {
		text-align: left;
		border-left: 1px solid #666;
		border-right: 1px solid #666;
		border-bottom: 1px solid #222;
		padding: 2px;
	}

                """ }
            }
            body {
                input
                    type="text"
                    hx-get="/query"
                    hx-trigger="change"
                    hx-swap="innerHTML"
                    hx-target="#tbody"
                    name="search"
                    { }
                table{
                    thead {
                        tr{
                            th{"name"}
                            th{"signature"}
                        }
                    }
                    tbody id="tbody" {}
                }
            }
        } }
        .into(),
    )
}

#[handler]
fn search(
    Query(TextSearch { search, take }): Query<TextSearch>,
    state: Data<&Arc<Table>>,
) -> Html<String> {
    let take = take.unwrap_or(5);
    let rows: Rows<'_> = state.compare(&search, take).into();
    Html( rows.render().into() )
}

use infag::*;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let state = fake_table().await;
    let app = Route::new()
        .at("/", get(index))
        .at("/query", get(search))
        .with(AddData::new(Arc::new(state)));

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .name("add-data")
        .run(app)
        .await
}

async fn fake_table() -> Table {
    let src_max = std::env::var("LEVSMAX")
        .map(|a| a.parse().ok())
        .ok()
        .flatten()
        .unwrap_or(30);
    Table::new(
        src_max,
        2,
        vec![
            vec!["<a href=\"https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop\">pop</a>",  "Vec&lt;T&gt; -> Option&lt;T&gt;"],
            vec!["<a href=\"https://example.com\">find</a>",  "Vec&lt;T&gt;,Fn(T)->bool -> Option&lt;T&gt;"],
            vec!["<a href=\"https://example.com\">first</a>", "Vec&lt;T&gt; -> Option&lt;T&gt;"            ],
            vec!["<a href=\"https://example.com\">take</a>",  "Option&lt;T&gt; -> Option&lt;T&gt;"         ],
            vec!["<a href=\"https://example.com\">ok_or</a>", "Option&lt;T&gt;, E -> Result&lt;T, E&gt;"   ],
        ]
        .into_iter()
        .map(|fnc| fnc.into_iter().map(String::from).collect())
        .collect(),
    )
    .expect("manual dev table")
}
