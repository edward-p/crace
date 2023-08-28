#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{quiz::{load_quiz, QuizType}, data::Data};

#[inline_props]
pub fn WrongList(cx: Scope) -> Element {
    
    let quiz_map=load_quiz(QuizType::All);

    render! {
        h4 { "错题列表"}
    }
}