use yew_router::prelude::*;
use yew::prelude::*;

// Routes

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

pub fn switch(routes: Route) -> Html {
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


