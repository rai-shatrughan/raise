use yew::prelude::*;

#[function_component(Videos)]
pub fn videos() -> Html {
    let items = vec!["Search", "News", "Videos", "Samsara"];
        html! {
            <div>
                <div class="pure-menu pure-menu-horizontal">
                    {                            
                        items.into_iter().map(|item| {
                            html!{
                                <a class="pure-menu-heading pure-menu-link" 
                                    href={item} 
                                    key={item}
                                    onClick={item}
                                    >{item}</a>
                                }
                        }).collect::<Html>()
                    } 
                </div>     
            </div>
        }
}