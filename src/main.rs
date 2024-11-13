// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use rand::prelude::IndexedRandom;
use serde::Deserialize;
use slint::SharedString;

slint::include_modules!();


#[derive(Deserialize, Clone)]
struct QuizQuestion {
    question: String,
    answers: Vec<String>,
    correct_answer: usize,
    explanation: Vec<String>
}

fn main() {
    let main_window = AppWindow::new().expect("Failed to create the UI");

    let quiz_content = fs::read_to_string("questions.json").expect("Unable to read file");
    let quiz_questions: Vec<QuizQuestion> = serde_json::from_str(&quiz_content).expect("Unable to load quiz questions");
    let quiz_clone = quiz_questions.clone();
    let cloned_main_window = main_window.clone_strong();

    let mut counter = 0;

        // grab a random question
        let mut rng = rand::thread_rng();
        let question = quiz_clone.choose(&mut rng).expect("No questions found");

        cloned_main_window.set_question(question.question.clone().into());
        cloned_main_window.set_answer_0(question.answers[0].clone().into());
        cloned_main_window.set_answer_1(question.answers[1].clone().into());
        cloned_main_window.set_answer_2(question.answers[2].clone().into());
        cloned_main_window.set_answer_3(question.answers[3].clone().into());

        let mut question_clone = question.clone();
        let cloned_main_window_inner = cloned_main_window.clone_strong();

        cloned_main_window.on_select_answer(move |selected_index: i32| {
            cloned_main_window_inner.set_explanation(question_clone.explanation[selected_index as usize].clone().into());
            if selected_index == question_clone.correct_answer as i32 {
                cloned_main_window_inner.set_counter(cloned_main_window_inner.get_counter() + 1);
                // move to the next question
                //main_window.set_explanation(question_clone.explanation[selected_index as usize].clone().into());
            
                let mut rng = rand::thread_rng();
                let question = quiz_clone.choose(&mut rng).expect("No questions found");
                question_clone = question.clone();
                cloned_main_window_inner.set_question(question.question.clone().into());
                cloned_main_window_inner.set_answer_0(question.answers[0].clone().into());
                cloned_main_window_inner.set_answer_1(question.answers[1].clone().into());
                cloned_main_window_inner.set_answer_2(question.answers[2].clone().into());
                cloned_main_window_inner.set_answer_3(question.answers[3].clone().into());
            } else {
                cloned_main_window_inner.set_counter(0);
            }
        });

    main_window.run().expect("Issue encountered while running the UI");
}
