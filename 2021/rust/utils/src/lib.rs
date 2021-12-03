use std::io::{self, BufRead};

#[inline(always)]
pub fn stdin_to_int_vec() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

#[inline(always)]
pub fn stdin_to_str_vec() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
