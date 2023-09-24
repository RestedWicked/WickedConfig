use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/lootbox")]
    Lootbox,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Lootbox => html! {
            <Lootbox />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1>},
    }
}

#[function_component(Home)]
fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Lootbox));
    html! {
        <div>
            <Heading />
            <button {onclick}>{ "Lootbox Generator" }</button>
        </div>
    }
}

#[function_component(Lootbox)]
fn lootbox() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <Heading />
            <div class={classes!("generator")}>
                <h2>{ "Lootbox Generator" }</h2>
                <button {onclick}>{ "Home" }</button>
            </div>
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
        
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

// Template Items

#[function_component(Heading)]
fn heading() -> Html {

    html! {
        <header>
            <h1> { "SurvivalQuest Admin Tools" } </h1>
        </header>
    }
}
