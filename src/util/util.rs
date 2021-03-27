use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn get_rand_char() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(1).map(char::from).collect()
}

pub fn get_rand_num(begin: usize, end: usize) -> usize {
    thread_rng().gen_range(begin..end)
}

pub fn string_ends_with_any(s: &str, suffixes: Vec<&str>) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix))
}
