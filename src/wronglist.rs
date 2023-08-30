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
                    h2{
                        "暂无错题..."
                    }
                };
            }

            let num_index = use_state(cx, || 0_usize);

            let quiz_next = quiz_list.get(num_index.get().clone()).unwrap().clone();

            let correct_list = use_state(cx, ||HashSet::new());

            let state = use_state(cx, || "".to_string());

            let disabled = use_state(cx, || "false".to_string());

            let jump_to = use_state(cx, || 1_usize);
            let quiz = use_state(cx, || quiz_next);

            render! {
                h4 { "错题列表 {num_index+1}/{quiz_list.len()}" },
                div{
                    b{
                        "{quiz.index}. {quiz.question}"
                    },
                    if !state.is_empty() {
                        render!{
                            blockquote {
                                style: " font-size: 0.9rem",
                                "{state}"
                            }
                        }
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
                div{
                    style: "display: flex; flex-wrap: wrap; gap: 1.5rem; margin-top: 1.5rem",
                    for i in 0..4 {
                        div {
                            button{
                                onclick: move |_| {
                                    if Choice::from(i) != quiz.answer  {
                                        log::info!("Wrong!");
                                        // add wrong indicator
                                        disabled.set("true".into());
                                        state.set(format!("❌ 回答：{}, 正确答案：{}", Choice::from(i).to_string(), quiz.answer.to_string()));
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
                                        state.set(format!("✔️ 回答：{}, 正确答案：{}", Choice::from(i).to_string(), quiz.answer.to_string()));
                                    }
                                },
                                disabled:"{disabled}",
                                style: "background: var(--accent-light); color: var(--text); text-align: left; font-size: 0.9rem",
                                "{Choice::from(i)}. {quiz.choice[i]}"
                            }
                        }
                    }

                },

                section{
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 1.5rem",
                        
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
                                quiz.set((&quiz_list).get(next_num_index).unwrap().clone());

                                let quiz_next=&quiz_list.get(next_num_index).unwrap();

                                if correct_list.contains(&quiz_next.index){
                                    disabled.set("true".into());
                                    state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer.to_string(), quiz_next.answer.to_string()));
                                }else{
                                    // clear state
                                    disabled.set("false".into());
                                    state.set("".into());
                                }
                            },
                            "跳转到"
                        },

                        if *num_index.get() > 0{
                            render!{
                                button {
                                    onclick: move |_| {
                                        let next_num_index=num_index-1;
                                        num_index.set(next_num_index);
                                        quiz.set(quiz_list.get(next_num_index).unwrap().clone());

                                        let quiz_next=quiz_list.get(next_num_index).unwrap();
                                        if correct_list.contains(&quiz_next.index){
                                            disabled.set("true".into());
                                            state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer.to_string(), quiz_next.answer.to_string()));
                                        }else{
                                            // clear state
                                            disabled.set("false".into());
                                            state.set("".into());
                                        }
                                    },
                                    "上一题"
                                },
                            }
                        }
                        if *num_index.get() < quiz_list.len() - 1 {
                            render!{
                                button {
                                onclick: move |_| {
                                    let next_num_index = num_index+1;
                                    num_index.set(next_num_index);
                                    let quiz_next=quiz_list.get(next_num_index).unwrap().clone();
                                    quiz.set(quiz_next);

                                    let quiz_next=quiz_list.get(next_num_index).unwrap();
                                    if correct_list.contains(&quiz_next.index){
                                        disabled.set("true".into());
                                        state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer.to_string(), quiz_next.answer.to_string()));
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
