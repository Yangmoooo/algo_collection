pub mod conditions;

use conditions::Poker;

pub fn sort(poker: &mut Poker) {
    let len = poker.len();
    for i in 1..len {
        for _ in 0..len - i {
            sink_bigger(poker);
        }
        for _ in 0..i {
            sink_smaller(poker);
        }
    }
    poker.sink();
}

fn sink_bigger(poker: &mut Poker) {
    let (first, second) = poker.check();
    if first < second {
        poker.swap();
    }
    poker.sink();
}

fn sink_smaller(poker: &mut Poker) {
    let (first, second) = poker.check();
    if first > second {
        poker.swap();
    }
    poker.sink();
}
