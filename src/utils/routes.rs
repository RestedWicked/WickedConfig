use crate::pages::home::Home;
use crate::pages::lootbox::Lootbox;

use yew::{ Html, html };
use yew_router::Routable;


// Routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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


