use crate::utils::routes::Route;
use crate::pages::components::heading::Heading;

use yew::{ Html, html, function_component, Callback};
use yew_router::hooks::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Lootbox));
    html! {
        <div>
            <Heading />
            <button {onclick}>{ "Lootbox Generator" }</button>
        </div>
    }
}
