use yew::{ Html, html, function_component};

#[function_component(Heading)]
pub fn heading() -> Html {

    html! {
        <header>
            <h1> { "SurvivalQuest Admin Tools" } </h1>
        </header>
    }
}
