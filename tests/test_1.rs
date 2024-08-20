use algo_collection::problem_1;

#[cfg(test)]
mod test_1 {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut answer = vec.clone();
        let mut poker = problem_1::conditions::Poker::new(vec);

        answer.sort();
        problem_1::sort(&mut poker);
        assert_eq!(poker.0, answer);
    }
}
