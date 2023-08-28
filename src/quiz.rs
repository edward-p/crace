use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use linked_hash_map::LinkedHashMap;
use rand::{distributions::Standard, prelude::Distribution, Rng};

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
            _ => Self::All
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

#[derive(Debug, Copy, Clone)]
pub enum Choice {
    A,
    B,
    C,
    D,
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0..=4) {
            0 => Choice::A,
            1 => Choice::B,
            2 => Choice::C,
            _ => Choice::D,
        }
    }
}

#[derive(Debug)]
pub struct Quiz {
    index: String,
    question: String,
    picture: String,
    choice: Vec<String>,
    answer: Choice,
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

pub fn load_quiz(quiz_type: QuizType) -> Result<LinkedHashMap<String, Quiz>, Box<dyn Error>> {
    let mut map = LinkedHashMap::new();

    let file = File::open(format!("resources/{}", quiz_type))?;
    let reader = BufReader::new(file);

    let mut q = Quiz::default();
    for li in reader.lines() {
        let line = li?;

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
                map.insert(q.index.clone(), q);
                q = Quiz::default();
            }
            _ => continue,
        }
    }

    Ok(map)
}

#[test]
fn test_read() {
    let res = load_quiz(QuizType::ClassB);
    if let Ok(map) = res {
        println!("{:?}", map);
    } else {
        assert!(false);
    }
}
