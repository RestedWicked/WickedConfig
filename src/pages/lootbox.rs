use crate::utils::routes::Route;
use crate::pages::components::heading::Heading;
use crate::pages::components::text_input::TextInput;

use yew::{ Html, html, function_component, Callback, classes, SubmitEvent};
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
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
    });

    html! {
        <form onsubmit={onsubmit}>
            <ul>
                <li>
                    <ul class={classes!("inline-textinput")}>
                        <TextInput name="lootbox_name" label="Lootbox Name"/>
                        <TextInput name="panel_name" label="Panel Name"/>
                    </ul>
                </li>
                <TextInput name="opening_stages" label="Opening Stages"/>
                <TextInput name="material_type" label="Material Type"/>
            </ul>
        </form>
    }
}
