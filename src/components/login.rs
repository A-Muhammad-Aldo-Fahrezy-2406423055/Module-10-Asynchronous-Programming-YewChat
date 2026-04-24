use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-slate-50 flex w-screen h-screen font-sans">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="bg-white p-12 rounded-3xl shadow-xl shadow-slate-200 flex flex-col items-center max-w-md w-full border border-slate-100">
                    <div class="w-16 h-16 bg-indigo-50 rounded-2xl flex items-center justify-center mb-8">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-indigo-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                        </svg>
                    </div>
                    <h1 class="text-3xl font-bold text-slate-900 mb-3 tracking-tight">{"YewChat"}</h1>
                    <p class="text-slate-400 mb-10 text-center text-sm leading-relaxed px-4">{"Experience real-time communication with a refined, minimal interface. Enter your name to begin."}</p>
                    <form class="flex flex-col w-full space-y-4">
                        <div class="relative">
                            <input {oninput} class="w-full rounded-2xl p-4 bg-slate-50 border border-slate-200 text-slate-700 placeholder:text-slate-300 focus:outline-none focus:ring-4 focus:ring-indigo-50 focus:border-indigo-200 transition-all duration-300 text-sm" placeholder="Your nickname" />
                        </div>
                        <Link<Route> to={Route::Chat}> 
                            <button {onclick} disabled={username.len()<1} class="w-full rounded-2xl bg-slate-900 hover:bg-indigo-600 text-white font-bold py-4 px-8 shadow-lg shadow-slate-200 transition-all duration-300 disabled:opacity-30 disabled:pointer-events-none active:scale-95" >
                                {"Enter Lounge"}
                            </button>
                        </Link<Route>>
                    </form>
                    <div class="mt-10 text-[11px] font-bold text-slate-300 uppercase tracking-[0.2em]">{"Powered by Rust & Yew"}</div>
                </div>
            </div>
        </div>
    }
}
