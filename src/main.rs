struct Machine;

enum BottleFillerState {
    Waiting { waiting_time: std::time::Duration },
    Filling { rate: usize },
    Done
}

struct BottleFiller {
    state: BottleFillerState
}


fn main() {
    // наш базовый автомат как обыкновенная структура, которую можно создать и уничтожить.
    let machine = Machine;
    // `machine` будет уничтожена, когда выйдет за пределы области видимости.
}
