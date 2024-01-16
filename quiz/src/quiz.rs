struct Question {
    title: String,
    answer: bool,
    user_answer: Option<bool>,
}

impl Question {
    fn is_answered(&self) -> bool {
        self.user_answer.is_some()
    }

    fn is_correct(&self) -> bool {
        self.user_answer == Some(self.answer)
    }
}

struct Quiz {
    questions: Vec<Question>,
    current_index: usize,
}

impl Quiz {
    fn current_question(&self) -> &Question {
        &self.questions[self.current_index]
    }

    fn next_question(&mut self) -> Question {
        let count = self.questions.len() - 1;
        if self.current_question().is_answered() && self.current_index < count {
            self.current_index += 1;
        }

        self.current_question()
    }

    fn score(&self) -> usize {
        // notice `mut`
        let mut correct = 0;
        for question in &self.questions {
            if question.is_correct() {
                correct += 1;
            }
        }

        correct
    }

    pub fn previous(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.questions.len() - 1;
            return;
        }

        self.current_index = self.current_index - 1;
    }

    /**
     * Sets the user's answer on the current question and returns if it's the correct
     * answer
     */
    pub fn answer(&mut self, answer: bool) -> bool {
        let question = &mut self.questions[self.current_index];
        question.user_answer = Some(answer);
        question.answer == answer
    }
}
