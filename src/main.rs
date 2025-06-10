use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/submitQuote")]
    SubmitQuote,
    #[at("/quotes")]
    Quotes,
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
            <li class="nav-item">
              <Link<Route> to={Route::SubmitQuote} classes="nav-link">{"Submit Quote"}</Link<Route>>
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Properties)]
pub struct Quote {
    pub author: Author,
    pub id: i32,
    pub quote: String,
    pub related_tags: Vec<Option<Tag>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthorResponse {
    pub pages: i32,
    pub authors: Vec<Author>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthorDetailResponse {
    pub author: Author,
    pub pages: i32,
    pub quotes: Vec<Quote>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TagResponse {
    pub pages: i32,
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TagDetailResponse {
    pub tag: Tag,
    pub pages: i32,
    pub quotes: Vec<Quote>,
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

#[derive(PartialEq, Properties, Clone)]
struct QuoteCardProps {
    quote: Quote,
}

#[derive(Properties, Clone, PartialEq, Deserialize, Serialize)]
struct AuthorWithQuotesProps {
    id: String,
}

#[derive(Properties, Clone, PartialEq, Deserialize, Serialize)]
struct TagWithQuotesProps {
    id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TagSubmission {
    tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QuoteSubmission {
    author_name: String,
    quote: String,
    related_tags: Vec<TagSubmission>,
}

#[function_component(QuoteSubmissionForm)]
fn quote_submission_form() -> Html {
    let author_name = use_state(|| "".to_string());
    let quote = use_state(|| "".to_string());
    let tag_input = use_state(|| "".to_string());
    let tags = use_state(Vec::<TagSubmission>::new);

    let on_author_input = {
        let author_name = author_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                author_name.set(input.value());
            }
        })
    };

    let on_quote_input = {
        let quote = quote.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                quote.set(input.value());
            }
        })
    };

    let on_tag_input = {
        let tag_input = tag_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                tag_input.set(input.value());
            }
        })
    };

    let on_add_tag = {
        let tag_input = tag_input.clone();
        let tags = tags.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let tag = tag_input.trim().to_string();
            if !tag.is_empty() {
                let mut new_tags = (*tags).clone();
                new_tags.push(TagSubmission { tag: tag.clone() });
                tags.set(new_tags);
                tag_input.set("".to_string());
            }
        })
    };

    let on_form_submit: Callback<SubmitEvent> = {
        let author_name = author_name.clone();
        let quote = quote.clone();
        let tags = tags.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let author_name = (*author_name).clone();
            let quote = (*quote).clone();
            let tags = (*tags).clone();

            wasm_bindgen_futures::spawn_local(async move {
                let submission = QuoteSubmission {
                    author_name,
                    quote,
                    related_tags: tags,
                };

                log::info!("Submitting: {:?}", submission);

                let client = reqwest::Client::new();
                match client
                    .post("http://127.0.0.1:3000/api/quotes")
                    .json(&submission)
                    .send()
                    .await
                {
                    Ok(resp) => log::info!("Submitted! Status: {:?}", resp.status()),
                    Err(err) => log::error!("Failed to submit: {:?}", err),
                }
            });
        })
    };

    html! {
        <form onsubmit={on_form_submit} class="container mt-4">
            <div class="mb-3">
                <label for="author_name" class="form-label">{ "Author Name" }</label>
                <input
                    type="text"
                    class="form-control"
                    id="author_name"
                    value={(*author_name).clone()}
                    oninput={on_author_input}
                />
            </div>

            <div class="mb-3">
                <label for="quote" class="form-label">{ "Quote" }</label>
                <input
                    type="text"
                    class="form-control"
                    id="quote"
                    value={(*quote).clone()}
                    oninput={on_quote_input}
                />
            </div>

            <div class="mb-3">
                <label for="tag" class="form-label">{ "Add Tag" }</label>
                <div class="input-group">
                    <input
                        type="text"
                        class="form-control"
                        id="tag"
                        value={(*tag_input).clone()}
                        oninput={on_tag_input}
                    />
                    <button class="btn btn-outline-secondary" onclick={on_add_tag}>{ "Add" }</button>
                </div>
            </div>

            <div class="mb-3">
                <label class="form-label">{ "Tags" }</label>
                <ul class="list-group">
                    { for tags.iter().map(|t| html! {
                        <li class="list-group-item">{ &t.tag }</li>
                    }) }
                </ul>
            </div>

            <button type="submit" class="btn btn-primary">{ "Submit Quote" }</button>
        </form>
    }
}
#[function_component(Authors)]
fn authors() -> Html {
    let authors = use_state(Vec::<Author>::new);
    let client = reqwest::Client::new();

    {
        let authors = authors.clone();

        use_effect_with((), move |_| {
            let authors = authors.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Fetching authors from the server...");

                let response = client
                    .get("http://127.0.0.1:3000/api/authors")
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                log::info!("Response received from the server.");

                let parsed_response = match response {
                    Ok(resp) => match resp.json::<AuthorResponse>().await {
                        Ok(data) => data,
                        Err(err) => {
                            log::error!("Failed to parse response: {}", err);
                            return;
                        }
                    },
                    Err(err) => {
                        log::error!("Failed to fetch authors: {}", err);
                        return;
                    }
                };
                authors.set(parsed_response.authors.into_iter().collect::<Vec<Author>>());
            });
            || ()
        });
    }

    html! {
        <>
            <h2>{"Authors"}</h2>
            <div class="list-group">
                { for authors.iter().map(|author| {
                    html! {
                        <Link<Route> to={Route::AuthorDetail { id: author.id.to_string() }} classes="list-group-item list-group-item-action">
                            { &author.name }
                        </Link<Route>>
                    }
                }) }
            </div>
        </>
    }
}

#[function_component(AuthorWithQuotes)]
fn author_with_quotes(props: &AuthorWithQuotesProps) -> Html {
    let client = reqwest::Client::new();
    let author = use_state(|| None);
    let quotes = use_state(Vec::<Quote>::new);
    {
        let author = author.clone();
        let quotes = quotes.clone();
        let author_id = props.id.clone();
        use_effect_with((), move |_| {
            let author = author.clone();
            let quotes = quotes.clone();
            let author_id = author_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Fetching author and their quotes from the server...");

                let response = client
                    .get(format!(
                        "http://127.0.0.1:3000/api/authors/{}",
                        author_id.clone()
                    ))
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                log::info!("Author Response received from the server.");

                let parsed_response = match response {
                    Ok(resp) => match resp.json::<AuthorDetailResponse>().await {
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

                quotes.set(parsed_response.quotes.into_iter().collect::<Vec<Quote>>());

                author.set(parsed_response.author.into());
            });
            || ()
        });
    }

    html! {
        <>
            <h2>{ format!("Quotes by {}", author.as_ref().map_or("Loading...".to_string(), |a| a.name.clone())) }</h2>
            <QuoteList quotes={(*quotes).clone()} />
        </>
    }
}

#[function_component(Tags)]
fn tags() -> Html {
    let tags = use_state(Vec::<Tag>::new);
    let client = reqwest::Client::new();

    {
        let tags = tags.clone();

        use_effect_with((), move |_| {
            let tags = tags.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Fetching tags from the server...");

                let response = client
                    .get("http://127.0.0.1:3000/api/tags")
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                log::info!("Response received from the server.");

                let parsed_response = match response {
                    Ok(resp) => match resp.json::<TagResponse>().await {
                        Ok(data) => data,
                        Err(err) => {
                            log::error!("Failed to parse response: {}", err);
                            return;
                        }
                    },
                    Err(err) => {
                        log::error!("Failed to fetch authors: {}", err);
                        return;
                    }
                };
                tags.set(parsed_response.tags.into_iter().collect::<Vec<Tag>>());
            });
            || ()
        });
    }

    html! {
        <>
            <h2>{"Tags"}</h2>
            <div class="list-group">
                { for tags.iter().map(|tag| {
                    html! {
                        <Link<Route> to={Route::TagDetail { id: tag.id.to_string() }} classes="list-group-item list-group-item-action">
                            { &tag.tag }
                        </Link<Route>>
                    }
                }) }
            </div>
        </>
    }
}

#[function_component(TagWithQuotes)]
fn tag_with_quotes(props: &TagWithQuotesProps) -> Html {
    let client = reqwest::Client::new();
    let tag = use_state(|| None);
    let quotes = use_state(Vec::<Quote>::new);
    {
        let tag = tag.clone();
        let quotes = quotes.clone();
        let tag_id = props.id.clone();
        use_effect_with((), move |_| {
            let tag = tag.clone();
            let quotes = quotes.clone();
            let tag_id = tag_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Fetching tag and related quotes from the server...");

                let response = client
                    .get(format!("http://127.0.0.1:3000/api/tags/{}", tag_id.clone()))
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                log::info!("Tag Response received from the server.");

                let parsed_response = match response {
                    Ok(resp) => match resp.json::<TagDetailResponse>().await {
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

                quotes.set(parsed_response.quotes.into_iter().collect::<Vec<Quote>>());

                tag.set(parsed_response.tag.into());
            });
            || ()
        });
    }

    html! {
        <>
            <h2>{ format!("Quotes with tag: {}", tag.as_ref().map_or("Loading...".to_string(), |t| t.tag.clone())) }</h2>
            <QuoteList quotes={(*quotes).clone()} />
        </>
    }
}

#[function_component(QuoteCard)]
fn quote_card(QuoteCardProps { quote }: &QuoteCardProps) -> Html {
    html! {
        <li class="list-group-item">
            <figure>
                <blockquote class="blockquote">
                    <p>{ &quote.quote}</p>
                </blockquote>
                <figcaption class="blockquote-footer">
                    <Link<Route> to={Route::AuthorDetail { id: quote.author.id.to_string() }}>{ format!("{}", quote.author.name) }</Link<Route>>
                </figcaption>
            </figure>
            <div class="card-footer text-muted">
                <span class="badge bg-secondary">{"Tags: "}</span>
                { for quote.related_tags.iter().filter_map(|tag| tag.as_ref()).map(|tag| html! {
                    <Link<Route> to={Route::TagDetail { id: tag.id.to_string() }} classes="badge bg-primary ms-1">{ &tag.tag }</Link<Route>>
                }) }
            </div>
        </li>
    }
}

#[function_component(QuoteList)]
fn quote_list(QuoteListProps { quotes }: &QuoteListProps) -> Html {
    html! {
        <ul class="list-group">
            { for quotes.iter().map(|quote| html! {

                <QuoteCard quote={quote.clone()} />
            }) }
        </ul>
    }
}

#[function_component(Quotes)]
fn quotes() -> Html {
    let client = reqwest::Client::new();
    let quotes = use_state(Vec::<Quote>::new);

    {
        let quotes = quotes.clone();
        use_effect_with((), move |_| {
            let quotes = quotes.clone();
            println!("Fetching quotes from the server...");
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Fetching quotes from the server...");

                let response = client
                    .get("http://127.0.0.1:3000/api/quotes")
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

                quotes.set(parsed_response.quotes.into_iter().collect::<Vec<Quote>>());
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
        Route::SubmitQuote => html! { <QuoteSubmissionForm /> },
        Route::Tags => html! { <Tags /> },
        Route::TagDetail { id } => html! { <TagWithQuotes id={id} /> },
        Route::Authors => html! { <Authors /> },
        Route::AuthorDetail { id } => html! { <AuthorWithQuotes id={id} /> },
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
