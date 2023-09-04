#![allow(non_snake_case)]
use crate::{
    data::Data,
    quiz::{load_exam, Choice, QuizType},
};
use dioxus::prelude::*;

#[inline_props]
pub fn ExamQuizList(cx: Scope, name: String) -> Element {
    let quiz_type=QuizType::from(name);
    let quiz_data = use_future(cx, (), |_| load_exam(quiz_type));
    // check if the future is resolved
    match quiz_data.value() {
        Some(Ok(quiz_list)) => {
            let num_index = use_state(cx, || 0);
            let quiz_next = quiz_list.get(*num_index.get()).unwrap().clone();
            let submitted = use_state(cx, || false);
            let jump_to = use_state(cx, || 1_usize);
            let quiz: &UseState<crate::quiz::Quiz> = use_state(cx, || quiz_next);
            let score_s = use_state(cx, || 0);
            let answer_s = use_ref(cx, || {
                let mut a: Vec<Option<Choice>> = Vec::new();
                for _ in 0..30 {
                    a.push(None);
                }
                a
            });

            render! {
                    div{
                    class: "quiz-content",
                    div{
                        class: "exam-head",
                        h5 { "{name}类模拟考试 {num_index+1}/{quiz_list.len()}" },

                        if *submitted.get(){
                            if *score_s.get() >= quiz_type.get_pass_score() {
                                render!(h5{style: "color: red", "通过, 得分: {score_s}"})
                            }else {
                                render!(h5{style: "color: red", "未通过, 得分: {score_s}"})
                            }
                        }else {
                            render!(
                                button {
                                    disabled: "{*num_index.get() < quiz_list.len() - 1}",
                                    onclick: move |_| {
                                        let mut data = Data::get_from_storage();
                                        let mut score = 0;
            
                                        for (i,quiz) in quiz_list.iter().enumerate(){
                                            if *answer_s.read().get(i).unwrap() == Some(quiz.answer){
                                                score += 1;
                                            }else {
                                                data.wrong_list.insert(quiz.index.clone());
                                            }
                                        }
                                        submitted.set(true);
                                        score_s.set(score);
                                        data.save();
                                    },
                                    "交卷"
                                }
                            )
                        }
                    }
                    div{
                        b{
                            "{quiz.index}. {quiz.question}"
                        },
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
                    if *submitted.get() {
                        match *answer_s.read().get(*num_index.get()).unwrap() {
                            Some(ans) if ans == quiz.answer => render!(blockquote {"✔️ 回答：{ans}, 正确答案：{quiz.answer}"}),
                            Some(ans) => render!(blockquote {"❌ 回答：{ans}, 正确答案：{quiz.answer}"}),
                            None=>  render!(blockquote {"❌ 回答：None, 正确答案：{quiz.answer}"}),
                        }
                    }
                    div{
                        class: "quiz-options",
                        for i in 0..4 {
                            div {
                                button{
                                    style: if Some(Choice::from(i))==*answer_s.read().get(*num_index.get()).unwrap(){
                                        "background: var(--accent); color: var(--bg);"
                                    }else {
                                        ""
                                    },
                                    onclick: move |_| {
                                        let mut answer = answer_s.write();
                                        let a = answer.get_mut(*num_index.get()).unwrap();
                                        *a = Some(Choice::from(i));
                                    },
                                    disabled:"{submitted}",
                                    "{Choice::from(i)}. {quiz.choice[i]}"
                                }
                            }
                        }
                    }
                }
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
                            let next_num_index = jump_to - 1;
                            num_index.set(next_num_index);
                            quiz.set(quiz_list.get(next_num_index).unwrap().clone());
                        },
                        "跳转"
                    },

                    button {
                        disabled: "{*num_index.get() <= 0}",
                        onclick: move |_| {
                            let next_num_index = num_index-1;
                            num_index.set(next_num_index);
                            quiz.set(quiz_list.get(next_num_index).unwrap().clone());
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
