#![allow(non_snake_case)]
use crate::{quiz::Quiz, quizlist::QuizList, wronglist::WrongList};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
            #[layout(ExamClass)]
                #[route("/")]
                ExamClass {},
            #[end_layout]
        #[end_nest]
        #[route("/:name")]
        QuizList { name: String },
        #[route("/wronglist")]
        WrongList {},
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
        nav {
            div {
                a { Link { to: Route::ExamClass {}, "顺序练习" } }
                a { Link { to: Route::WrongList {}, "错题列表" } }
            }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn ExamClass(cx: Scope) -> Element {
    render! {
        nav {
            a {
                Link {
                    to: Route::QuizList { name: "A".into()},
                    "A类考试"
                }
            }
            a {
                Link {
                    to: Route::QuizList { name: "B".into()},
                    "B类考试"
                }
            }
            a {
                Link {
                    to: Route::QuizList { name: "C".into()},
                    "C类考试"
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

pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
