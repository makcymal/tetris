use std::{
    io::{
        self, Read, Write, Error,
    },
    fs::File,
};


pub fn update_score(file: &mut File,
                    highscores: &[u32],
                    max_lines: &[u32]) -> Result<(), Error>
{
    let highscores = nums_to_string(highscores);
    let max_lines = nums_to_string(max_lines);

    file.write_all(format!("{highscores}\n{max_lines}\n").as_bytes())?;

    Ok(())
}


pub fn upload_score(file: &mut File) -> Option<(Vec<u32>, Vec<u32>)> {
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content) {
        return None;
    }

    let mut lines = content
        .split("\n")
        .take(2)
        .filter_map(|line| {
            let nums = string_to_nums(line);
            match nums.len() {
                0 => None,
                _ => Some(nums), 
            }
        })
        .collect::<Vec<_>>();

    if lines.len() != 2 {
        return None;
    }

    let mut highscores: Vec<u32>= vec!();
    let mut max_lines: Vec<u32>= vec!();

    std::mem::swap(&mut highscores, &mut lines[0]);
    std::mem::swap(&mut max_lines, &mut lines[1]);

    Some((highscores, max_lines))
}


fn nums_to_string(nums: &[u32]) -> String {
    nums
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}


fn string_to_nums(string: &str) -> Vec<u32> {
    string
        .split(" ")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect()
}
