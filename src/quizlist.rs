#![allow(non_snake_case)]
use crate::{
    data::Data,
    quiz::{load_quiz, Choice, QuizType},
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
            let state = use_state(cx, || "".to_string());
            let disabled = use_state(cx, || "false".to_string());
            let jump_to = use_state(cx, || 1_usize);
            render! {
                h4 { "{name}类考试 {num_index+1}/{quiz_list.len()}" },
                div{
                    b{
                        "{quiz.index}. {quiz.question}"
                    },
                    if !state.is_empty() {
                        render!{
                            blockquote {
                                "{state}"
                            }
                        }
                    }
                },
                if !quiz.picture.is_empty() {
                    render!{
                        div{
                            img{
                                style: "height: 20rem",
                                src:"/resources/pictures/{quiz.picture}"
                            }
                        },
                    }
                }
                div{
                    style: "display: flex; flex-wrap: wrap; gap: 2rem; margin-top: 2rem",
                    for i in 0..4 {
                        div {
                            button{
                                onclick: move |_| {
                                    if Choice::from(i) != quiz.answer  {
                                        log::info!("Wrong!");
                                        let mut data = Data::get_from_storage();
                                        data.wrong_list.insert(quiz.index.clone());
                                        data.save();
                                        // add wrong indicator
                                        state.set(format!("❌ 回答：{}, 正确答案：{}", Choice::from(i).to_string(), quiz.answer.to_string()));
                                    } else {
                                        log::info!("Correct!");
                                        // clear state
                                        disabled.set("true".into());
                                        state.set(format!("✔️ 回答：{}, 正确答案：{}", Choice::from(i).to_string(), quiz.answer.to_string()));
                                    }
                                },
                                disabled:"{disabled}",
                                style: "background: var(--accent-light); color: var(--text); text-align: left;",
                                "{Choice::from(i)}. {quiz.choice[i]}"
                            }
                        }
                    }

                },
                section{
                div {
                    style: "display: flex; bottom:0; gap: 1.5rem",
                    button {
                        onclick: move |_| {
                            let mut data = Data::get_from_storage();
                            num_index = jump_to - 1;
                            data.last_position.insert(name.clone(), num_index);
                            quiz.set(quiz_list.get(num_index).unwrap().clone());
                            data.save();
    
                            // clear button state
                            disabled.set("false".into());
                            state.set("".into());
                        },
                        "跳转到"
                    },
                    input{
                        placeholder:"输入题号",
                        r#type: "number",
                        min: "1",
                        max: "{quiz_list.len()}",
                        value: "{jump_to}",
                        oninput: move |evt| {
                            if let Ok(n) = evt.value.parse::<usize>(){
                                let j;
                                if n<1 {
                                    j=1;
                                }else if n>quiz_list.len(){
                                    j=quiz_list.len();
                                }else {
                                    j=n;
                                }
                                jump_to.set(j);
                            } else {
                                jump_to.set(1);
                            }
                        },
                    },


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
                                    state.set("".into());
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
                                state.set("".into());
                            },
                            "下一题"
                            }
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
