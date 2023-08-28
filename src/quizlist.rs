#![allow(non_snake_case)]
use crate::{
    data::Data,
    quiz::{self, load_quiz, Choice, QuizType, Quiz},
};
use dioxus::prelude::*;

#[inline_props]
pub fn QuizList(cx: Scope, name: String) -> Element {
    let quiz_data = use_future(cx, (), |_| load_quiz(QuizType::from(name)));
    let quiz=use_ref(cx, ||Quiz::default());
    // check if the future is resolved
    match quiz_data.value() {
        Some(Ok(quiz_data)) => {
            let mut data = Data::get_from_storage();
            let mut num_index = data.last_position.get(name).unwrap().clone();
            let mut iter = quiz_data.values().skip(num_index);
            quiz.set(iter.next().unwrap().clone());
            render! {
                h4 { "{name}类考试"},
                b{
                    "{num_index+1}. {quiz.read().index} {quiz.read().question}"
                },
                ul{
                    for i in 0..4 {
                        li{
                            "{Choice::from(i)}. {quiz.read().choice.get(i).unwrap()}"
                        }
                    }

                },
                if !quiz.read().picture.is_empty() {
                    render!{
                        div{
                            img{
                                src:"/resources/{quiz.read().picture}"
                            }
                        }
                    }
                }
                if num_index>1{
                    render!{
                        button {
                            "上一题"
                        },
                    }
                }
                b{
                    "{num_index+1}/{quiz_data.len()}"
                }
                if num_index<quiz_data.len()-1 {
                    render!{
                        button {
                         onclick: move |_| {
                            num_index=num_index+1;
                            data.last_position.insert(name.clone(), num_index);
                            data.save();
                            quiz.set(iter.next().unwrap().clone());
                         },
                         "下一题"
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
