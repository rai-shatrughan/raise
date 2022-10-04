mod home;
mod header;
mod search;
mod news;
mod videos;
mod nirvana;

use home::Home;

fn main() {
    yew::start_app::<Home>();
}