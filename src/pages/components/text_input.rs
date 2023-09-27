use yew::{ Html, html, function_component, Properties, AttrValue };

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {

    html! {
        <li>
            <label for={ props.name.clone() }> { props.label.clone() } </label>
            <input type="text" id={ props.name.clone() } name={ props.name.clone() } />
        </li>
    }
}

