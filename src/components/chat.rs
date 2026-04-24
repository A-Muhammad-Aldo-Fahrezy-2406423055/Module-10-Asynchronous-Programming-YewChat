use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::Link;
use std::collections::HashSet;

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, Route, User, MessageData};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleTranslate(usize),
    NoOp,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    translated_indices: HashSet<usize>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            translated_indices: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let username = user.username.borrow().clone();

        match msg {
            Msg::HandleMsg(s) => {
                if s.is_empty() {
                    return false;
                }
                match serde_json::from_str::<WebSocketMessage>(&s) {
                    Ok(msg) => match msg.message_type {
                        MsgTypes::Users => {
                            let users_from_message = msg.data_array.unwrap_or_default();
                            self.users = users_from_message
                                .iter()
                                .map(|u| UserProfile {
                                    name: u.into(),
                                    avatar: format!(
                                        "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                        u
                                    )
                                    .into(),
                                })
                                .collect();
                            true
                        }
                        MsgTypes::Message => {
                            if let Some(data_str) = msg.data {
                                if let Ok(message_data) = serde_json::from_str::<MessageData>(&data_str) {
                                    if message_data.from != username {
                                        user.messages.borrow_mut().push(message_data);
                                        return true;
                                    }
                                }
                            }
                            false
                        }
                        _ => false,
                    },
                    Err(_) => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let val = input.value();
                    if val.is_empty() { return false; }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(val.clone()),
                        data_array: None,
                    };

                    user.messages.borrow_mut().push(MessageData {
                        from: username.clone(),
                        message: val,
                    });

                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                    return true;
                };
                false
            }
            Msg::ToggleTranslate(idx) => {
                if self.translated_indices.contains(&idx) {
                    self.translated_indices.remove(&idx);
                } else {
                    self.translated_indices.insert(idx);
                }
                true
            }
            Msg::NoOp => false,
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let username = user.username.borrow().clone();
        let messages = user.messages.borrow();

        html! {
            <div class="flex w-screen h-screen bg-slate-50 font-sans text-slate-900">
                <div class="flex-none w-72 h-screen bg-white border-r border-slate-200 z-10 flex flex-col">
                    <div class="p-8">
                        <div class="text-xs font-bold uppercase tracking-widest text-slate-400 mb-6">{"Online Users"}</div>
                        <div class="space-y-4">
                        {
                            self.users.clone().iter().map(|u| {
                                html!{
                                    <div class="flex items-center group cursor-default">
                                        <div class="relative">
                                            <img class="w-10 h-10 rounded-full grayscale group-hover:grayscale-0 transition-all duration-300" src={u.avatar.clone()} alt="avatar"/>
                                            <div class="absolute -bottom-1 -right-1 w-3 h-3 bg-emerald-500 border-2 border-white rounded-full"></div>
                                        </div>
                                        <div class="ml-4 overflow-hidden">
                                            <div class="text-sm font-medium text-slate-700 group-hover:text-indigo-600 transition-colors">{u.name.clone()}</div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                        </div>
                    </div>
                    
                    <div class="mt-auto p-8 border-t border-slate-100">
                        <Link<Route> to={Route::Login}>
                            <button class="w-full flex items-center justify-center space-x-2 text-slate-400 hover:text-rose-500 transition-colors duration-200">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z" clip-rule="evenodd" />
                                </svg>
                                <span class="text-sm font-semibold tracking-wide uppercase">{"Sign Out"}</span>
                            </button>
                        </Link<Route>>
                    </div>
                </div>

                <div class="grow flex flex-col">
                    <header class="h-20 bg-white/80 backdrop-blur-md border-b border-slate-200 flex items-center justify-between px-10">
                        <div class="flex items-center space-x-3">
                            <div class="w-2 h-2 rounded-full bg-indigo-500 animate-pulse"></div>
                            <h2 class="text-lg font-semibold tracking-tight">{"YewChat"}</h2>
                        </div>
                        <div class="text-[13px] text-slate-400">
                            {"Session active as "} <span class="font-bold text-slate-800">{username.clone()}</span>
                        </div>
                    </header>
                    
                    <div class="grow overflow-y-auto p-10 space-y-8 scroll-smooth">
                        {
                            messages.iter().enumerate().map(|(idx, m)| {
                                let maybe_user = self.users.iter().find(|u| u.name == m.from);
                                let avatar = maybe_user.map(|u| u.avatar.clone()).unwrap_or_else(|| "https://avatars.dicebear.com/api/adventurer-neutral/default.svg".into());
                                let is_me = m.from == username;
                                let is_translated = self.translated_indices.contains(&idx);
                                
                                html!{
                                    <div class={classes!("flex", "w-full", if is_me { "justify-end" } else { "justify-start" })}>
                                        <div class={classes!("flex", "max-w-[75%]", if is_me { "flex-row-reverse" } else { "flex-row" })}>
                                            <div class={classes!("flex", "flex-col", if is_me { "items-end" } else { "items-start" }, if is_me { "ml-4" } else { "mr-4" })}>
                                                <img class="w-8 h-8 rounded-full shadow-sm" src={avatar} alt="avatar"/>
                                            </div>
                                            
                                            <div class={classes!("group", "relative", "flex", "flex-col", if is_me { "items-end" } else { "items-start" })}>
                                                <div class="text-[11px] font-bold text-slate-400 uppercase tracking-tighter mb-1.5 px-1">{m.from.clone()}</div>
                                                
                                                <div class={classes!("p-4", "rounded-2xl", "transition-all", "duration-300", 
                                                    if is_me { "bg-slate-800" } else { "bg-white" },
                                                    if is_me { "text-slate-50" } else { "text-slate-700" },
                                                    if is_me { "shadow-indigo-100" } else { "border" },
                                                    if is_me { "shadow-xl" } else { "border-slate-100" },
                                                    if !is_me { Some("shadow-sm") } else { None })}>
                                                    
                                                    if m.message.ends_with(".gif") {
                                                        <img class="rounded-lg max-w-sm" src={m.message.clone()}/>
                                                    } else {
                                                        <div class="relative">
                                                            <p class="text-[14px] leading-relaxed">
                                                                if is_translated {
                                                                    {format!("✨ [Smart Translation]: {}", m.message.chars().rev().collect::<String>())}
                                                                } else {
                                                                    {m.message.clone()}
                                                                }
                                                            </p>
                                                        </div>
                                                    }
                                                </div>

                                                <button 
                                                    onclick={ctx.link().callback(move |_| Msg::ToggleTranslate(idx))}
                                                    class={classes!("mt-2", "text-[10px]", "font-bold", "uppercase", "tracking-widest", "flex", "items-center", "space-x-1", "opacity-0", "group-hover:opacity-100", "transition-opacity", "duration-200",
                                                        if is_translated { "text-indigo-500" } else { "text-slate-400" },
                                                        if !is_translated { Some("hover:text-indigo-400") } else { None })}>
                                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor">
                                                        <path fill-rule="evenodd" d="M7 2a1 1 0 011 1v1h3a1 1 0 110 2H9.578a18.87 18.87 0 01-1.724 4.78c.29.354.596.696.914 1.026a1 1 0 11-1.44 1.389c-.188-.196-.373-.396-.554-.6a19.098 19.098 0 01-3.107 3.567 1 1 0 01-1.334-1.49 17.087 17.087 0 003.13-3.733 18.992 18.992 0 01-1.487-2.494 1 1 0 111.79-.89c.234.47.489.928.764 1.372.417-.934.752-1.913.997-2.927H3a1 1 0 110-2h3V3a1 1 0 011-1zm6 6a1 1 0 01.707.293l5.414 5.414a1 1 0 010 1.414l-5.414 5.414A1 1 0 0112 20V8a1 1 0 011-1z" clip-rule="evenodd" />
                                                    </svg>
                                                    <span>{if is_translated { "Show Original" } else { "Translate" }}</span>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    <footer class="p-8 bg-white border-t border-slate-200">
                        <div class="max-w-4xl mx-auto flex items-center bg-slate-50 border border-slate-200 rounded-2xl px-2 py-1.5 focus-within:bg-white focus-within:ring-4 focus-within:ring-indigo-50 focus-within:border-indigo-200 transition-all duration-300">
                            <input ref={self.chat_input.clone()} type="text" placeholder="Compose a message..." 
                                class="grow bg-transparent py-3 px-4 outline-none text-slate-700 placeholder:text-slate-400 text-sm" 
                                onkeydown={ctx.link().callback(|e: KeyboardEvent| if e.key() == "Enter" { Msg::SubmitMessage } else { Msg::NoOp })}
                                name="message" required=true />
                            <button onclick={submit} class="p-3.5 bg-slate-900 hover:bg-indigo-600 text-white rounded-xl shadow-lg transition-all duration-300 hover:-translate-y-0.5 ml-2">
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 fill-current">
                                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                    </footer>
                </div>
            </div>
        }
    }
}
