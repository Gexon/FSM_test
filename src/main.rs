// Общая для каждого состояния функциональность
trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

struct Waiting {
    waiting_time: std::time::Duration,
    // Данные, общие для всех состояний
    shared_value: usize
}

impl Waiting {
    fn new() -> Self {
        Waiting {
            waiting_time: std::time::Duration::new(0, 0),
            shared_value: 0
        }
    }
    // Поглощаем данные!
    fn to_filling(self) -> Filling {
        Filling {
            rate: 1,
            shared_value: 0
        }
    }
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}


struct Filling {
    rate: usize,
    // Общие для всех состояний данные
    shared_value: usize,
}

impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}


fn main() {
    let in_waiting_state = Waiting::new();
    let in_filling_state = in_waiting_state.to_filling();
}
