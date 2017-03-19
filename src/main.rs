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
    _rate: usize
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
                _rate: 1
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


// В случае, если вы делаете это внутри функции, сигнатура которой ограничивает выходной тип:
fn _transition_the_states(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
    val.into() // Мило, не правда ли?
}


fn main() {
    //let in_waiting = BottleFillingMachine::<Waiting>::new(0);
    //let _in_filling = BottleFillingMachine::<Filling>::from(in_waiting);

    let bottle_filler = BottleFillingMachine::new(0);

    // (Mock) Check on some shared and state-specific values
    assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));
    assert_eq!(bottle_filler.shared_value, 0);

    // Transition
    let bottle_filler = BottleFillingMachine::<Filling>::from(bottle_filler);

    // Can't do this anymore, it's been consumed!:
    // assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));

    let _bottle_filler = BottleFillingMachine::<Done>::from(bottle_filler);
}

// тесты:
// http://rurust.github.io/rust_book_ru/src/testing.html
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// попутно немного тесты попробую. ничего сложного)
// запускать так: cargo test
#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}