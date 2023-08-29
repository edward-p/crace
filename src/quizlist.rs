#![allow(non_snake_case)]
use crate::{
    data::Data,
    quiz::{load_quiz, QuizType, Choice},
};
use dioxus::prelude::*;

#[inline_props]
pub fn QuizList(cx: Scope, name: String) -> Element {
    let quiz_data = use_future(cx, (), |_| load_quiz(QuizType::from(name)));

    // check if the future is resolved
    match quiz_data.value() {
        Some(Ok(quiz_list)) => {
            let data = Data::get_from_storage();
            let mut num_index = data.last_position.get(name).unwrap().clone();
            let quiz = use_ref(cx, || quiz_list.get(num_index).unwrap().clone());
            let q = quiz.read();
            render! {
                h4 { "{name}类考试 {num_index+1}/{quiz_list.len()}" },
                b{
                    "{num_index+1}. {q.index} {q.question}"
                },
                if !q.picture.is_empty() {
                    render!{
                        div{
                            img{
                                src:"/resources/{q.picture}"
                            }
                        },
                    }
                }
                div{
                    for i in 0..4 {
                        div{
                            button{
                                style: "background: var(--accent-light); color: var(--text)",
                                "{Choice::from(i)}. {q.choice.get(i).unwrap()}"
                            }
                        }
                    }

                },
                div {
                    if num_index > 0{
                        render!{
                            button {
                                onclick: move |_| {
                                    let mut data = Data::get_from_storage();
                                    num_index = num_index-1;
                                    data.last_position.insert(name.clone(), num_index);
                                    quiz.set(quiz_list.get(num_index).unwrap().clone());
                                    data.save();
                                },
                                "上一题"
                            },
                        }
                    }
                    if num_index < quiz_list.len() - 1 {
                        render!{
                            button {
                            onclick: move |_| {
                                let mut data = Data::get_from_storage();
                                num_index = num_index+1;
                                data.last_position.insert(name.clone(), num_index);
                                quiz.set(quiz_list.get(num_index).unwrap().clone());
                                data.save();
                            },
                            "下一题"
                            }
                        }
                    }
                }


            }
        }
        Some(Err(err)) => {
            // if there was an error, render the error
            render! {"加载题目失败: {err}"}
        }
        None => {
            // if the future is not resolved yet, render a loading message
            render! {"正在加载题目..."}
        }
    }
}
