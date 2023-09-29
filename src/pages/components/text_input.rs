use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{ Html, html, function_component, Properties, AttrValue, Callback, Event };

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub handlechange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handlechange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });

    html! {
        <li>
            <label for={ props.name.clone() }> { props.label.clone() } </label>
            <input type="text" id={ props.name.clone() } name={ props.name.clone() } onchange={onchange} />
        </li>
    }
}

