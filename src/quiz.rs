use std::{error::Error, fmt::Display};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::data::Data;

#[derive(Debug)]
pub enum QuizType {
    ClassA,
    ClassB,
    ClassC,
    All,
}

impl From<&String> for QuizType {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "a" | "A" => Self::ClassA,
            "b" | "B" => Self::ClassB,
            "c" | "C" => Self::ClassC,
            _ => Self::All,
        }
    }
}

impl Display for QuizType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClassA => write!(f, "class_a.txt"),
            Self::ClassB => write!(f, "class_b.txt"),
            Self::ClassC => write!(f, "class_c.txt"),
            Self::All => write!(f, "class_all.txt"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Choice {
    A,
    B,
    C,
    D,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

impl From<usize> for Choice {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            _ => todo!(),
        }
    }
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0..=3) {
            0 => Choice::A,
            1 => Choice::B,
            2 => Choice::C,
            _ => Choice::D,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Quiz {
    pub index: String,
    pub question: String,
    pub picture: String,
    pub choice: Vec<String>,
    pub answer: Choice,
}

impl Quiz {
    pub fn update(&mut self, value: Self) {
        self.index = value.index;
        self.question = value.question;
        self.picture = value.picture;
        self.choice = value.choice;
        self.answer = value.answer;
    }
}

impl Default for Quiz {
    fn default() -> Self {
        Self {
            index: "".into(),
            question: "".into(),
            picture: "".into(),
            choice: vec![String::new(); 4],
            answer: rand::random::<Choice>(),
        }
    }
}

pub async fn load_quiz(quiz_type: QuizType) -> Result<Vec<Quiz>, Box<dyn Error>> {
    let mut list = Vec::new();

    let baseurl = web_sys::window().unwrap().origin();
    let url = format!("{}/resources/{}", baseurl, quiz_type);
    let text = reqwest::get(url).await?.text().await?;
    let mut q = Quiz::default();
    for li in text.lines() {
        let line = li;

        if line.len() < 3 {
            continue;
        }

        match &line[..3] {
            "[I]" => q.index = (&line[3..]).into(),
            "[Q]" => q.question = (&line[3..]).into(),
            "[A]" => q.choice[q.answer as usize] = (&line[3..]).into(),
            "[B]" | "[C]" | "[D]" => {
                let mut i = rand::random::<usize>() % 4;
                while !q.choice[i].is_empty() {
                    i = (i + 1) % 4;
                }
                q.choice[i] = (&line[3..]).into();
            }
            "[P]" => {
                q.picture = (&line[3..]).into();
                list.push(q.clone());
                q = Quiz::default();
            }
            _ => continue,
        }
    }

    Ok(list)
}


pub async fn load_wrong_list() -> Result<Vec<Quiz>, Box<dyn Error>> {
    let list_all = load_quiz(QuizType::All).await?;
    let data = Data::get_from_storage();
    
    let list = list_all
        .iter()
        .filter(|q| data.wrong_list.contains(&q.index))
        .cloned()
        .collect::<Vec<Quiz>>();
    Ok(list)
}