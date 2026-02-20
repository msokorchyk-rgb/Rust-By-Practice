use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn staircase(n: i32) {
    let n = n as usize;
    for i in 1..=n {
        let row = format!("{}{}", " ".repeat(n - i), "#".repeat(i));
        println!("{}", row);
    }
}

#[allow(dead_code)]
pub fn run_staircase() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    if let Some(Ok(line)) = stdin_iterator.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            staircase(n);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_staircase_logic() {
        let n: usize = 4;
        let i: usize = 1;
        let row = format!("{}{}", " ".repeat(n - i), "#".repeat(i));
        assert_eq!(row, "   #");
    }
}
