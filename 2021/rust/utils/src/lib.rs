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

pub fn line_of_stdin_to_int_vec() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|s| {
            s.split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
