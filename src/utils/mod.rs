use rand::{seq::SliceRandom, thread_rng};

pub fn collect_input(input: String) -> Vec<i32> {
    input
        .trim()
        .split(&[' ', ','][..])
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn generate_new_numbers(input: &Vec<i32>, take_nbr: usize) -> Vec<i32> {
    let mut cloned_input = input.to_owned();

    cloned_input.shuffle(&mut thread_rng());
    cloned_input
        .iter()
        .take(take_nbr)
        .map(|x| *x)
        .collect::<Vec<i32>>()
}
