use yew::prelude::*;

#[function_component(Nirvana)]
pub fn nirvana() -> Html {
    let items = vec!["Search", "News", "Videos", "Nirvana"];
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