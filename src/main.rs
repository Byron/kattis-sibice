use std::{io, str::FromStr, io::Read};

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.split('\n');
    match lines.next() {
        Some(first) => {
            let mut nums = first.split(' ').take(3).map(usize::from_str);
            let (num_matches, w, h) = match (nums.next(), nums.next(), nums.next()) {
                (Some(a), Some(b), Some(c)) => (a?, b?, c?),
                _ => panic!("exhausted first line")
            };
            let max_size = (w.pow(2) as f64 + h.pow(2) as f64).sqrt();
            for match_len in lines.take(num_matches).map(usize::from_str) {
                let match_len = match_len?;
                if match_len as f64 > max_size {
                    println!("NE")
                } else {
                    println!("DA")
                }
            }
        },
        _ => panic!("exhausted")
    }
    Ok(())
}
