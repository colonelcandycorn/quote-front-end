use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/quotes")]
    Quotes,
    #[at("/quotes/:id")]
    QuoteDetail { id: String },
    #[at("/tags")]
    Tags,
    #[at("/tags/:id")]
    TagDetail { id: String },
    #[at("/")]
    Home,
    #[at("/authors")]
    Authors,
    #[at("/authors/:id")]
    AuthorDetail { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Nav)]
fn navbar() -> Html {
    html! {
<nav class="navbar navbar-expand-lg bg-body-tertiary">
  <div class="container-fluid">
    <Link<Route> to={Route::Quotes} classes="navbar-brand">{"Quote Server"}</Link<Route>>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarSupportedContent">
      <ul class="navbar-nav me-auto mb-2 mb-lg-0">
        <li class="nav-item">
          <Link<Route> to={Route::Quotes} classes="nav-link active">{"Quotes"}</Link<Route>>
        </li>
        <li class="nav-item dropdown">
            <Link<Route> to={Route::Tags} classes="nav-link">{"Tags"}</Link<Route>>
        </li>
        <li class="nav-item">
        <Link<Route> to={Route::Authors} classes="nav-link">{"Authors"}</Link<Route>>
        </li>
      </ul>
    </div>
  </div>
</nav>


    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QuoteResponse {
    pub pages: i32,
    pub quotes: Vec<Quote>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Quote {
    pub author: Author,
    pub id: i32,
    pub quote: String,
    pub related_tags: Vec<Option<Tag>>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Properties, Clone, PartialEq)]
struct QuoteListProps {
    quotes: Vec<Quote>,
}

#[function_component(QuoteList)]
fn quote_list(QuoteListProps { quotes }: &QuoteListProps) -> Html {
    html! {
        <ul class="list-group">
            { for quotes.iter().map(|quote| html! {
                <li class="list-group-item">
                    <Link<Route> to={Route::QuoteDetail { id: quote.id.to_string() }} classes="nav-link">
                        { format!("{} - {}", quote.id, quote.quote) }
                    </Link<Route>>
                </li>
            }) }
        </ul>
    }
}


#[function_component(Quotes)]
fn quotes() -> Html {
    let client = reqwest::Client::new();
    let quotes = use_state(|| vec![]);

    {
        let quotes = quotes.clone();
        use_effect_with((), move |_| {
            let quotes = quotes.clone();
            println!("Fetching quotes from the server...");
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Fetching quotes from the server...");
  

                let response = client.get("http://127.0.0.1:3000/api/quotes")
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                log::info!("Response received from the server.");

                let parsed_response = match response {
                    Ok(resp) => match resp.json::<QuoteResponse>().await {
                        Ok(data) => data,
                        Err(err) => {
                            log::error!("Failed to parse response: {}", err);
                            return;
                        }
                    },
                    Err(err) => {
                        log::error!("Failed to fetch quotes: {}", err);
                        return;
                    }
                };

                quotes.set(
                    parsed_response
                    .quotes
                    .into_iter()
                    .collect::<Vec<Quote>>()
                );
            });
            || ()
        });
    }
    html! { 
        <div>
            <h2>{"Quotes"}</h2>
            <QuoteList quotes={(*quotes).clone()} />
        </div>
     }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Home Page"}</h1> },
        Route::Quotes => html! { <Quotes /> },
        Route::QuoteDetail { id } => html! { <h1>{format!("Quote Detail for ID: {}", id)}</h1> },
        Route::Tags => html! { <h1>{"Tags Page"}</h1> },
        Route::TagDetail { id } => html! { <h1>{format!("Tag Detail for ID: {}", id)}</h1> },
        Route::Authors => html! { <h1>{"Authors Page"}</h1> },
        Route::AuthorDetail { id } => html! { <h1>{format!("Author Detail for ID: {}", id)}</h1> },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! { 
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}