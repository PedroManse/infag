use maud::{html, Render, DOCTYPE};
use poem::{
    endpoint::StaticFilesEndpoint,
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
                link rel="stylesheet" href="/files/css/style.css" {}
                script src="https://unpkg.com/htmx.org@1.9.11"{}
            }
            body {
               form
                    hx-get="/query"
                    hx-swap="innerHTML"
                    hx-target="#tbody"
                    hx-include="#search, #take"
                    hx-trigger="load, keyup change delay:0.2s, change"
                {
                    input
                        type="text"
                        name="search"
                        id="search"
                        { }
                    input
                        type="number"
                        name="take"
                        value="5"
                        id="take"
                        { }
                }
                table{
                    thead {
                        tr{
                            th{"name"}
                            th{"signature"}
                            th{"rustdoc"}
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
    let search = format!("<pre>{}</pre>", html_escape::encode_text(&search));
    let rows: Rows<'_> = state.compare(&search, take).into();
    Html(rows.render().into())
}

use infag::*;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let state = fake_table().await;
    let app = Route::new()
        .at("/", get(index))
        .at("/query", get(search))
        .nest(
            "/files",
            StaticFilesEndpoint::new("./files").show_files_listing(),
        )
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
        .unwrap_or(80);
    Table::new(
        src_max,
        3,
        vec![
            vec![
                "<b class=\"fn\">pop</b>",
                "<pre>Vec&lt;T&gt; -&gt; Option&lt;T&gt;</pre>",
                "<a href=\"https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop\">
                <b class=\"scope\">std</b>::\
                <b class=\"scope\">vec</b>::\
                <b class=\"lastscope\">Vec</b>.\
                <b class=\"fn\">pop</b></a>",
            ],
            vec![
                "<b class=\"fn\">find</b>",
                "<pre>Iterator&lt;Item=T&gt;,Fn(T)-&gt;bool -&gt; Option&lt;T&gt;</pre>",
                "<a href=\"https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find\">
                    <b class=\"scope\">std</b>::\
                    <b class=\"scope\">iter</b>::\
                    <b class=\"lastscope\">Iterator</b>.\
                    <b class=\"fn\">find</b></a>",
            ],
            vec![
                "<b class=\"fn\">first</b>",
                "<pre>Vec&lt;T&gt; -> Option&lt;T&gt;</pre>",
                "<a href=\"https://doc.rust-lang.org/std/primitive.slice.html#method.first\">
                <b class=\"lastscope\">slice</b>.\
                <b class=\"fn\">first</b></a>",
            ],
            vec![
                "<b class=\"fn\">take</b>",
                "<pre>Option&lt;T&gt; -> Option&lt;T&gt;</pre>",
                "<a href=\"https://doc.rust-lang.org/std/option/enum.Option.html#method.take\">
                <b class=\"scope\">std</b>::\
                <b class=\"scope\">option</b>::\
                <b class=\"lastscope\">Option</b>.\
                <b class=\"fn\">take</b></a>",
            ],
            vec![
                "<b class=\"fn\">ok_or</b>",
                "<pre>Option&lt;T&gt;, E -> Result&lt;T, E&gt;</pre>",
                "<a href=\"https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or\">
                    <b class=\"scope\">std</b>::\
                    <b class=\"scope\">option</b>::\
                    <b class=\"lastscope\">Option</b>.\
                    <b class=\"fn\">ok_or</b></a>",
            ],
        ]
        .into_iter()
        .map(|fnc| fnc.into_iter().map(String::from).collect())
        .collect(),
    )
    .expect("manual dev table")
}

/*
<b class="fn">{name}</b>
<pre>{signature}</pre>
<a href="...">
[<b class="scope">{scope[0..len-1]}</b>]
<b class="lastscope">{scope[len-1]}</b>
<b class="fn">{name}</b>
</a>
*/
