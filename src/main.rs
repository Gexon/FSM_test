// Конечный автомат в состоянии «Заполнение» будет BottleStateMachine<Filling>
struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S
}

// Следующие состояния могут быть `S` в StateMachine<S>
struct Waiting {
    waiting_time: std::time::Duration
}

struct Filling {
    rate: usize
}

struct Done;


impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value: shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0)
            }
        }
    }
}


impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Filling {
                rate: 1
            }
        }
    }
}


impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Done
        }
    }
}


fn main() {
    let in_waiting = BottleFillingMachine::<Waiting>::new(0);
    let in_filling = BottleFillingMachine::<Filling>::from(in_waiting);
}