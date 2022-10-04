use yew::prelude::*;
use crate::header::Header;
use crate::search::Search;

pub enum Msg {}
pub struct Home {}

impl Component for Home {    

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Header />
                <Search />
            </div>
        }
    }
}
