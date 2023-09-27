pub mod routes;

use routes::{ switch , Route };

use yew::{ Html, html, function_component};
use yew_router::{ BrowserRouter, Switch };


#[function_component]
pub fn App() -> Html {
        
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
