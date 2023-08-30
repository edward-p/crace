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

            let num_index = use_state(cx, || *data.last_position.get(name).unwrap());
            let quiz_next = quiz_list.get(*num_index.get()).unwrap().clone();

            let state = use_state(cx, || {
                if data.correct_list.contains(&quiz_next.index){
                    format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer, quiz_next.answer)
                }else{
                    "".into()
                }
            });

            let disabled = use_state(cx, || {
                if data.correct_list.contains(&quiz_next.index){
                    "true".to_string()
                }else{
                    "false".to_string()
                }
            });
            
            let jump_to = use_state(cx, || 1_usize);

            let quiz = use_state(cx, || quiz_next);

            render! {
                div{
                style: "max-height: 26rem; overflow: scroll",
                h5 { "{name}类考试 {num_index+1}/{quiz_list.len()}" },
                div{
                    b{
                        style: " font-size: 0.9rem;",
                        "{quiz.index}. {quiz.question}"
                    },
                },
                if !quiz.picture.is_empty() {
                    render!{
                        div{
                            img{
                                style: "width: 80%; height: auto",
                                src:"/resources/pictures/{quiz.picture}"
                            }
                        },
                    }
                }
                if !state.is_empty() {
                    render!{
                        blockquote {
                            style: " font-size: 0.9rem",
                            "{state}"
                        }
                    }
                }
                div{
                    style: "gap: 1.12rem; margin-top: 1.5rem",
                    for i in 0..4 {
                        div {
                            button{
                                onclick: move |_| {
                                    if Choice::from(i) != quiz.answer  {
                                        log::info!("Wrong!");
                                        let mut data = Data::get_from_storage();
                                        data.wrong_list.insert(quiz.index.clone());
                                        data.correct_list.remove(&quiz.index);
                                        data.save();
                                        // add wrong indicator
                                        disabled.set("true".into());
                                        state.set(format!("❌ 回答：{}, 正确答案：{}", Choice::from(i), quiz.answer));
                                    } else {
                                        log::info!("Correct!");
                                        let mut data = Data::get_from_storage();
                                        data.correct_list.insert(quiz.index.clone());
                                        data.save();
                                        // clear state
                                        disabled.set("true".into());
                                        state.set(format!("✔️ 回答：{}, 正确答案：{}", Choice::from(i), quiz.answer));
                                    }
                                },
                                disabled:"{disabled}",
                                style: "background: var(--bg); color: var(--text); text-align: left; font-size: 0.9rem; width: 80%",
                                "{Choice::from(i)}. {quiz.choice[i]}"
                            }
                        }
                    }

                }
            }
                section{
                style:"position: fixed; bottom:0",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 1.5rem",
                    input{
                        // style: "width: 65hw",
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
                            let mut data = Data::get_from_storage();
                            let next_num_index = jump_to - 1;
                            num_index.set(next_num_index);
                            data.last_position.insert(name.clone(), next_num_index);
                            quiz.set(quiz_list.get(next_num_index).unwrap().clone());
                            data.save();
    
                            let quiz_next=quiz_list.get(next_num_index).unwrap();
                            if data.correct_list.contains(&quiz_next.index){
                                disabled.set("true".into());
                                state.set(format!("✔️ 回答：{}, 正确答案：{}", quiz_next.answer, quiz_next.answer));
                            }else{
                                // clear state
                                disabled.set("false".into());
                                state.set("".into());
                            }
                        },
                        "跳转"
                    },


                    if *num_index.get() > 0{
                        render!{
                            button {
                                onclick: move |_| {
                                    let mut data = Data::get_from_storage();
                                    let next_num_index = num_index-1;
                                    num_index.set(next_num_index);
                                    data.last_position.insert(name.clone(), next_num_index);
                                    quiz.set(quiz_list.get(next_num_index).unwrap().clone());
                                    data.save();

                                    let quiz_next=quiz_list.get(next_num_index).unwrap();
                                    if data.correct_list.contains(&quiz_next.index){
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
                        }
                    }
                    if *num_index.get() < quiz_list.len() - 1 {
                        render!{
                            button {
                            onclick: move |_| {
                                let mut data = Data::get_from_storage();
                                let next_num_index = num_index+1;
                                num_index.set(next_num_index);
                                data.last_position.insert(name.clone(), next_num_index);
                                let quiz_next=quiz_list.get(next_num_index).unwrap().clone();
                                quiz.set(quiz_next);
                                data.save();

                                let quiz_next=quiz_list.get(next_num_index).unwrap();
                                if data.correct_list.contains(&quiz_next.index){
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
