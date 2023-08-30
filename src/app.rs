#![allow(non_snake_case)]
use crate::{quizlist::QuizList, wronglist::WrongList, data::Data};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        PracticeList {},
        #[route("/practice/:name")]
        QuizList { name: String },
        #[route("/wronglist")]
        WrongList {},
        #[route("/clear")]
        ClearStorage{},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
// ANCHOR_END: router

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        section{
            style:"position: fixed; top:0",
            header{
                nav {
                    Link { to: Route::PracticeList {}, "顺序练习" },
                    Link { to: Route::WrongList {}, "错题列表" },
                    Link { to: Route::ClearStorage {}, "清除练习记录" },
                }
            }
        }
        section{
        style:"position: fixed; top: 5rem",
        Outlet::<Route> {}
        }
    }
}

#[inline_props]
fn PracticeList(cx: Scope) -> Element {
    render! {
        div{
            style:"margin-top: 1.12rem",
        header {
            nav {
                Link {
                    to: Route::QuizList { name: "A".into()},
                    "A类考试"
                },
                Link {
                    to: Route::QuizList { name: "B".into()},
                    "B类考试"
                },
                Link {
                    to: Route::QuizList { name: "C".into()},
                    "C类考试"
                }
            }
        }
        }
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

#[inline_props]
fn ClearStorage(cx: Scope) -> Element {
    Data::clear();
    render! {
        h2 { "已清除练习记录!" }
    }
}

pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
