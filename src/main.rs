// Общая для каждого состояния функциональность
trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}


enum State {
    Waiting { waiting_time: std::time::Duration },
    Filling { rate: usize },
    Done
}

struct StateMachine { state: State }

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Waiting { waiting_time: std::time::Duration::new(0, 0) }
        }
    }
    fn to_filling(&mut self) {
        self.state = match self.state {
            // Только переход "Ожидание" -> "Заполнение" возможен
            State::Waiting { .. } => State::Filling { rate: 1 },
            // Остальные вызовут ошибку
            _ => panic!("Invalid state transition!")
        }
    }
    // ...
}

fn main() {
    let mut state_machine = StateMachine::new();
    state_machine.to_filling();

}
