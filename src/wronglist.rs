#![allow(non_snake_case)]
use std::collections::HashSet;

use crate::{
    data::Data,
    quiz::{load_wrong_list, Choice},
};
use dioxus::prelude::*;

#[inline_props]
pub fn WrongList(cx: Scope) -> Element {
    let quiz_data = use_future(cx, (), |_| load_wrong_list());
    // check if the future is resolved
    match quiz_data.value() {
        Some(Ok(quiz_list)) => {
            if quiz_list.is_empty() {
                return render! {
                    h4{
                        "暂无错题..."
                    }
                };
            }

            let num_index = use_state(cx, || 0_usize);

            let quiz_next = quiz_list.get(*num_index.get()).unwrap().clone();

            let correct_list = use_state(cx, HashSet::new);

            let state = use_state(cx, || "".to_string());

            let disabled = use_state(cx, || "false".to_string());

            let jump_to = use_state(cx, || 1_usize);
            let quiz = use_state(cx, || quiz_next);

            render! {
                div{
                    class: "quiz-content",
                h5 { "错题列表 {num_index+1}/{quiz_list.len()}" },
                div{
                    b{
                        "{quiz.index}. {quiz.question}"
                    }
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
                if !state.is_empty() {
                    render!{
                        blockquote {
                            "{state}"
                        }
                    }
                }
                div{
                    class: "quiz-options",
                    for i in 0..4 {
                        div {
                            button{
                                style: if "" != state.get() && i==(quiz.answer as usize) {
                                    "background: var(--accent); color: var(--bg);"
                                }else {
                                    ""
                                },
                                onclick: move |_| {
                                    if Choice::from(i) != quiz.answer  {
                                        log::info!("Wrong!");
                                        // add wrong indicator
                                        disabled.set("true".into());
                                        state.set(format!("❌ 回答：{}, 正确答案：{}", Choice::from(i), quiz.answer));
                                    } else {
                                        log::info!("Correct!");
                                        let mut list=correct_list.get().clone();
                                        list.insert(quiz.index.clone());
                                        correct_list.set(list);
                                        // remove from wrong_list
                                        let mut data = Data::get_from_storage();
                                        data.wrong_list.remove(&quiz.index);
                                        data.save();
                                        // clear state
                                        disabled.set("true".into());
                                        state.set(format!("✔️ 回答：{}, 正确答案：{}", Choice::from(i), quiz.answer));
                                    }
                                },
                                disabled:"{disabled}",
                                "{Choice::from(i)}. {quiz.choice[i]}"
                            }
                        }
                    }

                }
            },
                div {
                    class: "quiz-bottom",
                    input{
                        placeholder:"输入题号",
                        r#type: "number",
                        min: "1",
                        max: "{quiz_list.len()}",
                        value: "{jump_to}",
                        onchange: move |evt| {
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
                    button {
                        onclick: move |_| {
                            let next_num_index=jump_to - 1;
                            num_index.set(next_num_index);
                            quiz.set(quiz_list.get(next_num_index).unwrap().clone());

                            let quiz_next=&quiz_list.get(next_num_index).unwrap();

                            if correct_list.contains(&quiz_next.index){
                                disabled.set("true".into());
                                state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer, quiz_next.answer));
                            }else{
                                // clear state
                                disabled.set("false".into());
                                state.set("".into());
                            }
                        },
                        "跳转到"
                    },
                    button {
                        disabled: "{*num_index.get() <= 0}",
                        onclick: move |_| {
                            let next_num_index=num_index-1;
                            num_index.set(next_num_index);
                            quiz.set(quiz_list.get(next_num_index).unwrap().clone());

                            let quiz_next=quiz_list.get(next_num_index).unwrap();
                            if correct_list.contains(&quiz_next.index){
                                disabled.set("true".into());
                                state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer, quiz_next.answer));
                            }else{
                                // clear state
                                disabled.set("false".into());
                                state.set("".into());
                            }
                        },
                        "上一题"
                    },
                    button {
                        disabled: "{*num_index.get() >= quiz_list.len() - 1}",
                        onclick: move |_| {
                            let next_num_index = num_index+1;
                            num_index.set(next_num_index);
                            let quiz_next=quiz_list.get(next_num_index).unwrap().clone();
                            quiz.set(quiz_next);

                            let quiz_next=quiz_list.get(next_num_index).unwrap();
                            if correct_list.contains(&quiz_next.index){
                                disabled.set("true".into());
                                state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer, quiz_next.answer));
                            }else{
                                // clear state
                                disabled.set("false".into());
                                state.set("".into());
                            }
                        },
                        "下一题"
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
