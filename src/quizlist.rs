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
            let quiz = use_state(cx, || quiz_list.get(num_index).unwrap().clone());
            let state = use_state(cx, || vec!["";4]);
            // let quiz_r = quiz.read();
            let disabled= use_state(cx, || "false".to_string());
            render! {
                h4 { "{name}类考试 {num_index+1}/{quiz_list.len()}" },
                b{
                    "{num_index+1}. {quiz.index} {quiz.question}"
                },
                if !quiz.picture.is_empty() {
                    render!{
                        div{
                            img{
                                src:"/resources/pictures/{quiz.picture}"
                            }
                        },
                    }
                }
                div{
                    for i in 0..4 {
                        div {
                            button{
                                onclick: move |_| {
                                    let mut state_new = state.get().clone();
                                    if Choice::from(i) != quiz.answer  {
                                        log::info!("Wrong!");
                                        let mut data = Data::get_from_storage();
                                        data.wrong_list.insert(quiz.index.clone());
                                        data.save();
                                        // add wrong indicator
                                        state_new[i]="❌".into();
                                        state.set(state_new);
                                    } else {
                                        log::info!("Correct!");
                                        // clear state
                                        let mut state_new=vec!["";4];
                                        state_new[i]="✔️".into();
                                        disabled.set("true".into());
                                        state.set(state_new);
                                    }
                                },
                                disabled:"{disabled}",
                                style: "background: var(--accent-light); color: var(--text)",
                                "{Choice::from(i)}. {quiz.choice[i]}"
                            }
                            span {
                                style: "padding-left: 2rem",
                                "{state[i]}"
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

                                    // clear button state
                                    disabled.set("false".into());
                                    state.set(vec!["";4]);
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

                                // clear button state
                                disabled.set("false".into());
                                state.set(vec!["";4]);
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
