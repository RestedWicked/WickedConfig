mod utils;
mod pages;

use crate::utils::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
