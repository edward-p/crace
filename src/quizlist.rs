#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{quiz::{load_quiz, QuizType}, data::Data};

#[inline_props]
pub fn QuizList(cx: Scope, name: String) -> Element {
    
    let quiz_map=load_quiz(QuizType::from(name));

    render! {
        h2 { "{name}类考试"}
    }
}