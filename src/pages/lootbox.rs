use crate::utils::routes::Route;
use crate::pages::components::heading::Heading;
use crate::pages::components::text_input::TextInput;

use yew::{ Html, html, function_component, Callback, classes, use_state, SubmitEvent};
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
    let preview_lootbox_name = use_state(|| "Preview".to_owned());
    let cloned_lootbox_name = preview_lootbox_name.clone();
    let lootbox_name_onchange = Callback::from(move |lootbox_name| {
        cloned_lootbox_name.set(lootbox_name);
    });

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
    });

    html! {
        <form onsubmit={onsubmit}>
            <ul>
                <TextInput name="lootbox_name" label="Lootbox Name" handlechange={lootbox_name_onchange} />
                <li><p id="lootbox_name_preview"> { &*preview_lootbox_name } </p></li>
            </ul>
        </form>
    }
}
