use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/lootbox")]
    Lootbox,
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
            <h1>{ "SurvivalQuest Admin Tools" }</h1>
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
            <h1>{ "Lootbox Generator" }</h1>
            <button {onclick}>{ "Home" }</button>
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
