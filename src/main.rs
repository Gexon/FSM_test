// Конечный автомат в состоянии «Заполнение» будет BottleStateMachine<Filling>
struct BottleFillingMachine<S> {
    shared_value: usize,
    _state: S
}

// Следующие состояния могут быть `S` в StateMachine<S>
struct Waiting {
    _waiting_time: std::time::Duration
}

struct Filling {
    _rate: usize
}

struct Done;


impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value: shared_value,
            _state: Waiting {
                _waiting_time: std::time::Duration::new(0, 0)
            }
        }
    }
}


impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            _state: Filling {
                _rate: 1
            }
        }
    }
}


impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            _state: Done
        }
    }
}


// В случае, если вы делаете это внутри функции, сигнатура которой ограничивает выходной тип:
fn _transition_the_states(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
    val.into() // Мило, не правда ли?
}


fn main() {
    let in_waiting = BottleFillingMachine::<Waiting>::new(0);
    let _in_filling = BottleFillingMachine::<Filling>::from(in_waiting);
}