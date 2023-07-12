use rand::{seq::SliceRandom, thread_rng};

pub fn string_to_vec(input: Option<String>) -> Vec<String> {
    if input.is_some() {
        let value = input.unwrap();

        value
            .trim()
            .split(&[' ', ','][..])
            .filter(|x| x.parse::<String>().is_ok() && x != &"0" && x != &"")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    } else {
        vec![]
    }
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
