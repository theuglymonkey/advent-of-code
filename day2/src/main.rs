use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

fn read_file_vec(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    println!("With text:\n{data}");
    Ok(data)
}

fn read_file_vec_2(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(filepath)).unwrap();
    let reader = BufReader::new(&file);
    let lines = Ok(reader.lines().filter_map(io::Result::ok).collect());
    //println!("With text:\n{data}");
    lines
}

// Rock AX 1
// Paper BY 2
// Scissors CZ 3

//0 = loss
//3 = draw
//6 = win

// A X
// A Y
// A Z
// B X
// B Y
// B Z
// C X
// C Y
// C Z

// X lose
// Y Draw
// Z Win

fn main() {
    let _strat = read_file_vec_2("strat.txt");

    let mut scores = HashMap::new();
    let mut scores2 = HashMap::new();

    scores.insert(String::from("A X"), 3 + 1); // Choose C to lose
    scores.insert(String::from("A Y"), 6 + 2); // choose A for draw
    scores.insert(String::from("A Z"), 0 + 3); // Choose B to win
    scores.insert(String::from("B X"), 1);
    scores.insert(String::from("B Y"), 3 + 2);
    scores.insert(String::from("B Z"), 6 + 3);
    scores.insert(String::from("C X"), 6 + 1);
    scores.insert(String::from("C Y"), 0 + 2);
    scores.insert(String::from("C Z"), 3 + 3);

    scores2.insert(String::from("A X"), String::from("A Z"));
    scores2.insert(String::from("A Y"), String::from("A X"));
    scores2.insert(String::from("A Z"), String::from("A Y"));
    scores2.insert(String::from("B X"), String::from("B X"));
    scores2.insert(String::from("B Y"), String::from("B Y"));
    scores2.insert(String::from("B Z"), String::from("B Z"));
    scores2.insert(String::from("C X"), String::from("C Y"));
    scores2.insert(String::from("C Y"), String::from("C Z"));
    scores2.insert(String::from("C Z"), String::from("C X"));

    let mut totalScore = 0;
    let mut totalScore2 = 0;

    for strat in _strat.unwrap() {
         totalScore += scores.get(&strat).unwrap();
         let instruction = scores2.get(&strat).unwrap();
         totalScore2 += scores.get(instruction).unwrap()
    }

    println!("{}", totalScore);
    println!("{}", totalScore2);
 
}
