use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Params, PartialEq, PartialOrd)]
struct ContactParams {
    id: String,
}

#[derive(Params)]
struct ContactSearch {
    q: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages st ylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/encyclopedia.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/users" view=UserPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(AddTwo, "/api")]
pub async fn adding_two(x: i32, y: i32) -> Result<i32, ServerFnError> {
    Ok(x + y)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Home"</h1>
        <p>"Hello world"</p>
        <br></br>
        <p>"Hello world " {move || count.get() * 2}</p>
        <nav style="text-align: center;">
            <a href="/users">USER</a>
            <div style="display: inline-block; margin-right: 15px"></div>
            <a href="/login">LOGIN</a>
        </nav>
        <p>"Hello world"</p>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn UserPage() -> impl IntoView {
    // reactive access to URL query strings
    let query = use_query_map();
    // search stored as ?q=
    let search = move || {
        query()
            .get("q")
            .cloned()
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or_default()
    };
    // a resource driven by the search string
    let search_results = create_resource(search, move |_| adding_two(search(), search()));

    let add_two: MultiAction<AddTwo, Result<i32, ServerFnError>> =
        create_server_multi_action::<AddTwo>();

    let params = use_params::<ContactParams>();
    let id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.id.clone())
                .unwrap_or("asdas".to_string())
        })
    };

    let res = create_resource(move || add_two.version().get(), move |_| adding_two(1, 2));
    view! {
        <a href="/">Click</a>
        <div>
        <Transition
          fallback=move || view! {  <p>"Loading..."</p>}
        >
          {move || {
            res
            }
          }
        </Transition>
      </div>

      <div>
        <Form method="GET" action="">
        <input type="search" name="q" value=search/>
        </Form>
        <Transition
          fallback=move || view! {  <p>"Loading..."</p>}
        >
          {move || {
            search_results
            }
          }
        </Transition>
      </div>

        <div>
        <Transition
          fallback=move || view! {  <p>"Loading..."</p>}
        >
          {move || {
            search
            }
          }
        </Transition>
      </div>

        <div>
        <Transition
          fallback=move || view! {  <p>"Loading..."</p>}
        >
          {move || {
            id
            }
          }
        </Transition>
      </div>

    }
}

// Login page
#[component]
fn LoginPage() -> impl IntoView {
    // reactive access to URL query strings
    let query = use_query_map();
    // search stored as ?q=
    let search = move || {
        query()
            .get("q")
            .cloned()
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or_default()
    };
    // a resource driven by the search string
    let search_results = create_resource(search, move |_| adding_two(search(), search()));

    let add_two: MultiAction<AddTwo, Result<i32, ServerFnError>> =
        create_server_multi_action::<AddTwo>();

    view! {
        <h1>"Login"</h1>
        <br></br>
        <div>
        <Form method="GET" action="">
        <h3>Name</h3>
        <input type="search" name="q" value=search/>
        </Form>

        <div>
        <h3>Password</h3>
        <input type="password" name="p" value=search/>
        </div>

        <br></br>

        <Transition fallback=move || view! {  <p>"Loading..."</p>}>
          {move || {
            search_results
            }
          }
        </Transition>

      </div>
    }
}
