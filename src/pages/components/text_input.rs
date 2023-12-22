use wasm_bindgen::JsCast;
use web_sys::{ HtmlInputElement, KeyboardEvent };
use yew::{ Html, html, function_component, Properties, AttrValue, Callback, use_state };

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub preview: Option<bool>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let preview_input = use_state(|| "Preview".to_owned());
    let cloned_input = preview_input.clone();
    
    let _input_name = &props.name;
    let input_onchange = Callback::from(move |input_name| {
        cloned_input.set(input_name);
    });

    let onkey = Callback::from(move |event: KeyboardEvent| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        input_onchange.emit(value);
    });


    html! {
        <li>
            <label for={ props.name.clone() }> { props.label.clone() } </label>
            if props.preview.is_none() {
                <input type="text" id={ props.name.clone() } name={ props.name.clone() }/>
            }
            if props.preview.is_some() {
                <input type="text" id={ props.name.clone() } name={ props.name.clone() } onkeyup={onkey} />
                <p class="input_preview"> { &*preview_input } </p>
            }
        </li>
    }
}

