use colored::Colorize;
use rand::seq::SliceRandom;
use std::{
    f64::RADIX,
    io::{self, BufReader, Write},
    process::exit,
};

const LOGO: &str = r#"                                                                      
  .---.   ___  ___  ___     .--.    
 / .-, \ (   )(   )(   )  /  _  \   
(__) ; |  | |  | |  | |  . .' `. ;  
  .'`  |  | |  | |  | |  | '   | |  
 / .'| |  | |  | |  | |  _\_`.(___) 
| /  | |  | |  | |  | | (   ). '.   
; |  ; |  | |  ; '  | |  | |  `\ |  
' `-'  |  ' `-'   `-' '  ; '._,' '  
`.__.'_.   '.__.'.__.'    '.___.'   
"#;
const SUBLOGO: &str = r#"                                    
  ____ _                 _   ____                 _   _ _   _                       
 / ___| | ___  _   _  __| | |  _ \ _ __ __ _  ___| |_(_) |_(_) ___  _ __   ___ _ __ 
| |   | |/ _ \| | | |/ _` | | |_) | '__/ _` |/ __| __| | __| |/ _ \| '_ \ / _ \ '__|
| |___| | (_) | |_| | (_| | |  __/| | | (_| | (__| |_| | |_| | (_) | | | |  __/ |   
 \____|_|\___/ \__,_|\__,_| |_|   |_|  \__,_|\___|\__|_|\__|_|\___/|_| |_|\___|_|   
 / _ \ _   _(_)____                                                                 
| | | | | | | |_  /                                                                 
| |_| | |_| | |/ /                                                                  
 \__\_\\__,_|_/___|                                                                                               
"#;

#[derive(serde::Deserialize, Debug, Clone)]
struct Question {
    question: String,           // i.e. 'What is the answer to this question?'
    answers: Vec<String>,       // i.e. ['A. Answer 1', 'B. Answer 2']
    correct_answers: Vec<char>, // i.e. ['A', 'B']
    source: String,
    part: usize,
    uuid: String,
    question_number: usize,
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ProgressState {
    already_answered_uuids: Vec<String>,
    wrong_answered_uuids: Vec<String>,
    times: Vec<f64>,
}

fn save_progress(already_answered: Vec<Question>, wrong_answered: Vec<Question>, times: Vec<f64>) {
    // save the already answered questions to a file
    let home_path = dirs::home_dir().expect("Error getting home directory");
    let home_path = home_path.to_str().unwrap();
    let dir_path = home_path.to_string() + "/.aws-clf-c02-quiz";
    // create the directory if it doesn't exist
    std::fs::create_dir_all(dir_path).expect("Error creating directory");
    let file_path = home_path.to_string() + "/.aws-clf-c02-quiz/progress.json";
    let file = std::fs::File::create(file_path).expect("Error creating file");

    let progress_state = ProgressState {
        already_answered_uuids: already_answered.iter().map(|q| q.uuid.clone()).collect(),
        wrong_answered_uuids: wrong_answered.iter().map(|q| q.uuid.clone()).collect(),
        times,
    };

    serde_json::to_writer(file, &progress_state).expect("Error writing file");
}

fn load_progress(
) -> Result<(Vec<Question>, Vec<Question>, usize, usize, Vec<f64>), Box<dyn std::error::Error>> {
    // load the already answered questions from a file
    let home_path = dirs::home_dir().expect("Error getting home directory");
    let home_path = home_path.to_str().unwrap();
    let file_path = home_path.to_string() + "/.aws-clf-c02-quiz/progress.json";
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let progress_state: ProgressState = serde_json::from_reader(reader)?;

    let questions = load_questions();
    let already_answered: Vec<Question> = questions
        .iter()
        .filter(|q| progress_state.already_answered_uuids.contains(&q.uuid))
        .cloned()
        .collect();
    let wrong_answered: Vec<Question> = questions
        .iter()
        .filter(|q| progress_state.wrong_answered_uuids.contains(&q.uuid))
        .cloned()
        .collect();

    let total_count = already_answered.len();
    let correct_count = total_count - progress_state.wrong_answered_uuids.len();

    Ok((
        already_answered,
        wrong_answered,
        correct_count,
        total_count,
        progress_state.times,
    ))
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
    println!("\x1B[2J\x1B[1;1H");
    println!("{}", LOGO.bright_yellow());
    println!("{}", SUBLOGO.bright_cyan());

    println!(
        "{}",
        "Loading questions from the internet...\nPlease wait...".bright_yellow()
    );

    let mut all_questions = load_questions();
    println!("\x1B[2J\x1B[1;1H");

    println!("{}", LOGO.bright_yellow());
    println!("{}", SUBLOGO.bright_cyan());
    println!("Initialized: {} questions", all_questions.len());

    println!(
        "Press {} to start or type {} to quit.",
        "⏎ enter".purple(),
        ":q⏎".bright_red()
    );
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    if user_input.starts_with(":q") {
        exit(0);
    }
    if user_input.starts_with(":r") {
        all_questions = load_questions();
    }

    let (mut already_answered, mut wrong_answered, mut correct_count, mut total_count, mut times) =
        match load_progress() {
            Ok((already_answered, wrong_answered, correct_count, total_count, times)) => {
                if already_answered.len() > 0 {
                    (
                        already_answered,
                        wrong_answered,
                        correct_count,
                        total_count,
                        times,
                    )
                } else {
                    (Vec::new(), Vec::new(), 0, 0, Vec::new())
                }
            }
            Err(error) => {
                println!("Error loading progress: {}", error);
                (Vec::new(), Vec::new(), 0, 0, Vec::new())
            }
        };

    // questions should contain all questions that have been answered in the beginning,
    // followed by the rest of the questions, with the index pointing to the first unanswered question
    let mut questions: Vec<Question> = all_questions
        .iter()
        .filter(|q| !already_answered.contains(q))
        .cloned()
        .collect();

    loop {
        let mut start_timestamp: std::time::Instant;
        'outer: while questions.len() > 0 {
            let random_question = questions.remove(0);
            // clear the screen
            print!("\x1B[2J\x1B[1;1H");
            let termsize::Size { rows: _, cols } = termsize::get().unwrap();
            // print progress out of total questions
            let total_rate = total_count as f32 / all_questions.len() as f32;
            let str = format!(
                "{}/{} ({}%)",
                total_count,
                all_questions.len(),
                (total_rate * 100.0).round()
            );
            let str2: String;
            if total_count > 0 {
                let correct_rate = correct_count as f32 / total_count as f32;
                str2 = format!(
                    "{}/{} ({}%)",
                    correct_count,
                    total_count,
                    (correct_rate * 100.0).round()
                );
            } else {
                str2 = "0/0 (0%)".to_string();
            }
            let strmaxlen = if str.len() > str2.len() {
                str.len()
            } else {
                str2.len()
            };
            let cols_ = cols as usize - 11 - strmaxlen;
            let finished_signs = (total_rate * cols_ as f32).round() as usize;
            let unfinished_signs = (cols_ as usize) - finished_signs;
            print!("Progress: ");
            print!("{}", "█".repeat(finished_signs));
            print!("{}", "█".repeat(unfinished_signs).dimmed());
            print!(" {}", str);
            println!("");

            // print correct rate
            if total_count > 0 {
                let correct_rate = correct_count as f32 / total_count as f32;
                // print a bar showing the correct rate (using green and red)
                let cols_ = cols as usize - 11 - strmaxlen;
                let correct_signs = (correct_rate * cols_ as f32).round() as usize;
                let incorrect_signs = (cols_ as usize) - correct_signs;
                print!("Correct:  ");
                print!("{}", "█".repeat(correct_signs).green());
                print!("{}", "█".repeat(incorrect_signs).red());
                print!(" {}\n\n", str2);

                let avg_time = times.iter().sum::<f64>() / times.len() as f64;
                let median_time = {
                    let mut sorted_times = times.clone();
                    sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let mid = sorted_times.len() / 2;
                    if sorted_times.len() % 2 == 0 {
                        (sorted_times[mid - 1] + sorted_times[mid]) / 2.0
                    } else {
                        sorted_times[mid]
                    }
                };
                let max_time = times.iter().fold(0.0_f64, |a, &b| a.max(b));
                let min_time = times.iter().fold(100000.0_f64, |a, &b| a.min(b));
                let str3 = format!(
                    "⏳ Time: 🔻 Min: {:.2}s, 🔶 Median: {:.2}s, ➖ Average: {:.2}s, 🔺 Max: {:.2}s",
                    min_time, median_time, avg_time, max_time
                );
                // center the text
                let str3maxlen = str3.len();
                if str3maxlen > cols as usize {
                    println!("{}", str3);
                } else {
                    let cols_ = cols as usize - str3maxlen;
                    let half = cols_ / 2;
                    println!("{}{}", " ".repeat(half), str3);
                }
            }

            print!("\n\n");
            start_timestamp = std::time::Instant::now();
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
            for answer in random_question.answers.iter() {
                println!("{}", answer);
            }
            println!("\n");
            // let the user type one or more answers (separated by comma) (i.e. "A", "A,B")
            let mut user_selection: Vec<char>;
            loop {
                print!("{}", "Answer: ".cyan());
                io::stdout().flush().unwrap();
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input).unwrap();
                if user_input.starts_with(":q") {
                    exit(0);
                }
                if user_input.starts_with(":r") {
                    break 'outer;
                }
                let user_input = user_input.trim().replace(",", "").to_uppercase();
                user_selection = user_input.chars().collect();
                let number_of_answers = random_question.answers.len();
                let accepted_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
                let accepted_letters = &accepted_letters[..number_of_answers];
                if user_selection.len() != 0
                    && user_selection.len() == random_question.correct_answers.len()
                    && user_selection.iter().all(|c| accepted_letters.contains(c))
                {
                    break;
                }
                print!(
                    "Please enter {} answer",
                    random_question.correct_answers.len()
                );
                if random_question.correct_answers.len() > 1 {
                    println!(
                        "s (separated by comma or without any separator, in range A-{})",
                        accepted_letters[number_of_answers - 1]
                    );
                } else {
                    println!(" (in range A-{})", accepted_letters[number_of_answers - 1]);
                }
            }
            times.push(start_timestamp.elapsed().as_millis() as f64 / 1000.0);
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
                wrong_answered.push(random_question.clone());
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
            // save the progress
            // only the already answered questions are saved
            already_answered.push(random_question);

            save_progress(
                already_answered.clone(),
                wrong_answered.clone(),
                times.clone(),
            );
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
        println!(
            "All questions have been answered! Press {} to restart or {} to quit.",
            ":r⏎".purple(),
            ":q⏎".bright_red()
        );
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        if user_input.starts_with(":q") {
            exit(0);
        }
        exit(0)
    }
}
