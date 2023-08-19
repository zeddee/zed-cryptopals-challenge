use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    /*
    let mut res: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for lp in lines {
            if let Ok(content) = lp {
                res.push(content)
            }
        }
    };
    res
     */
    read_lines(filename)
        .expect("Could not read file")
        .map(|lp| {
            if let Ok(content) = lp {
                content
            } else {
                "".to_string()
            }
        })
        .collect()
}
