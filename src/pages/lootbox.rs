use crate::app::routes::Route;
use crate::pages::components::heading::Heading;
use crate::pages::components::text_input::TextInput;

use yew::{ Html, html, function_component, Callback, classes};
use yew_router::hooks::use_navigator;

#[function_component(Lootbox)]
pub fn lootbox() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <Heading />
            <div class={classes!("generator")}>
                <h2>{ "Lootbox Generator" }</h2>
                <button {onclick}>{ "Home" }</button>
            </div>
            <LootboxForm />
        </div>
    }
}

#[function_component(LootboxForm)]
fn lootbox_form() -> Html {

    html! {
        <form>
            <ul>
                <TextInput name="lootbox_name" label="Lootbox Name" />
            </ul>
        </form>
    }
}
