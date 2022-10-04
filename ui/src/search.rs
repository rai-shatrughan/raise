use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    html! {            
        <div class="container">
            <div class="row">
                <div class="col-6">
                    <input id="search-any" type="search" placeholder="search" />
                </div>
            </div>
            <div class="row">
                <div class="col-12">
                    <button id="submit-any" type="submit" class="pure-button">{"Search"}</button>
                </div>
            </div>
        </div>
    }
}
