use colored::Colorize;
use rand::seq::SliceRandom;
use std::{
    io::{self, BufReader, Write},
    process::exit,
};

#[derive(serde::Deserialize, Debug)]
struct Question {
    question: String,           // i.e. 'What is the answer to this question?'
    answers: Vec<String>,       // i.e. ['A. Answer 1', 'B. Answer 2']
    correct_answers: Vec<char>, // i.e. ['A', 'B']
    source: String,
    part: usize,
    question_number: usize,
}

fn load_questions() -> Vec<Question> {
    // load the newest questions from "https://raw.githubusercontent.com/MrLeaw/aws-clf-c02-quiz/refs/heads/main/all.json"
    let resp = reqwest::blocking::get(
        "https://raw.githubusercontent.com/MrLeaw/aws-clf-c02-quiz/refs/heads/main/all.json",
    )
    .expect("Error fetching latest questions, please check your internet connection");
    let reader = BufReader::new(resp);

    let mut questions: Vec<Question> = serde_json::from_reader(reader).expect("Error reading file");
    questions.shuffle(&mut rand::thread_rng());
    questions
}

fn main() {
    let mut correct_count = 0;
    let mut total_count = 0;
    let mut questions = load_questions();
    // Do something with the questions
    // print first question
    println!("\x1B[2J\x1B[1;1H");
    println!("Welcome to the quiz!");
    println!("Initialized: {} questions", questions.len());

    println!(
        "Press {} to start or type {} to quit.",
        "⏎ enter".purple(),
        ":q⏎".bright_red()
    );
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    if user_input == ":q\n" {
        exit(0);
    }
    if user_input == ":r\n" {
        questions = load_questions();
    }

    loop {
        'outer: for random_question in questions.iter() {
            // clear the screen
            print!("\x1B[2J\x1B[1;1H");
            let termsize::Size { rows, cols } = termsize::get().unwrap();
            // print correct rate
            let correct_rate = (correct_count as f32 / total_count as f32);
            if total_count > 0 {
                let str = format!(
                    "{}/{} ({}%)",
                    correct_count,
                    total_count,
                    (correct_rate * 100.0).round()
                );
                // print a bar showing the correct rate (using green and red)
                let cols = cols as usize - 10 - str.len();
                let correct_signs = (correct_rate * cols as f32).round() as usize;
                let incorrect_signs = (cols as usize) - correct_signs;
                print!("Correct: ");
                print!("{}", "█".repeat(correct_signs).green());
                print!("{}", "█".repeat(incorrect_signs).red());
                print!(" {}", str);
            } else {
                println!("");
            }

            print!("\n\n");

            println!(
                "{} {}",
                "Question ID:".cyan(),
                format!(
                    "{}-{}-{}",
                    random_question.source, random_question.part, random_question.question_number
                )
                .bright_yellow()
            );
            println!("{}\n", random_question.question);
            for (i, answer) in random_question.answers.iter().enumerate() {
                println!("{}", answer);
            }
            println!("\n");
            // let the user type one or more answers (separated by comma) (i.e. "A", "A,B")
            print!("{}", "Answer: ".cyan());
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            if user_input == ":q\n" {
                exit(0);
            }
            if user_input == ":r\n" {
                break 'outer;
            }
            let user_input = user_input.trim().replace(",", "").to_uppercase();
            let mut user_selection: Vec<char> = user_input.chars().collect();
            while user_selection.len() == 0
                || user_selection.len() != random_question.correct_answers.len()
            {
                print!(
                    "Please enter {} answer",
                    random_question.correct_answers.len()
                );
                if random_question.correct_answers.len() > 1 {
                    println!("s separated by comma");
                } else {
                    println!("");
                }
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input).unwrap();
                if user_input == ":q\n" {
                    break;
                }
                if user_input == ":r\n" {
                    break 'outer;
                }
                let user_input = user_input.trim().replace(",", "").to_uppercase();
                user_selection = user_input.chars().collect();
            }
            let mut correct = true;
            for answer in &random_question.correct_answers {
                if !user_selection.contains(answer) {
                    correct = false;
                    break;
                }
            }
            // print correct in green or incorrect in red
            total_count += 1;
            if correct {
                println!("{}", "Correct!".green());
                correct_count += 1;
            } else {
                println!("{}", "Incorrect!".red());
                // print correct answer(s) split by comma
                println!(
                    "Correct answer(s): {}",
                    random_question
                        .correct_answers
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );
            }
            // press any key to continue
            println!("Press {} to continue...", "⏎ enter".purple());
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            if user_input == ":q\n" {
                exit(0);
            }
            if user_input == ":r\n" {
                break 'outer;
            }
        }
        questions = load_questions();
    }
}
