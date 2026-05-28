enum AutoRunState {
    Idle,
    Inputting(String),
    Running(u32),
}

pub struct AutoRun {
    state: AutoRunState,
}

impl AutoRun {
    pub fn new() -> Self {
        AutoRun {
            state: AutoRunState::Idle,
        }
    }

    pub fn start_input(&mut self) {
        self.state = AutoRunState::Inputting(String::new());
    }

    pub fn push_digit(&mut self, d: char) {
        if let AutoRunState::Inputting(ref mut s) = self.state
            && s.len() < 7
        {
            s.push(d);
        }
    }

    pub fn pop_digit(&mut self) {
        if let AutoRunState::Inputting(ref mut s) = self.state {
            s.pop();
        }
    }

    pub fn confirm(&mut self) -> bool {
        if let AutoRunState::Inputting(ref s) = self.state
            && let Ok(n) = s.parse::<u32>()
            && n > 0
        {
            self.state = AutoRunState::Running(n);
            return true;
        }
        false
    }

    pub fn cancel(&mut self) {
        self.state = AutoRunState::Idle;
    }

    pub fn is_inputting(&self) -> bool {
        matches!(self.state, AutoRunState::Inputting(_))
    }

    pub fn input_digits(&self) -> Option<&str> {
        if let AutoRunState::Inputting(ref s) = self.state {
            Some(s)
        } else {
            None
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, AutoRunState::Running(_))
    }

    pub fn decrement(&mut self) {
        if let AutoRunState::Running(ref mut n) = self.state {
            *n = n.saturating_sub(1);
            if *n == 0 {
                self.state = AutoRunState::Idle;
            }
        }
    }
}
