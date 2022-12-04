use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

fn read_file_vec(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(filepath)).unwrap();
    let reader = BufReader::new(&file);
    let lines = Ok(reader.lines().filter_map(io::Result::ok).collect());
    //println!("With text:\n{data}");
    lines
}

// Read in string vec
// divide count of string by 2 to get items in each sack
// When getting priorirty if num is lower case subtract 141 if upper subtract 101
//

fn part1() {
    let sacks = read_file_vec("sacks.txt");
    let mut total_score = 0;

    for item in sacks.unwrap() {
        let mut string_map = HashMap::new();
        let split_string = item.split_at(item.len()/2);

        for i in split_string.1.chars() {
            let upper = i.is_uppercase();
            if upper {
                string_map.insert(i, 26 + (i as u32 - 64));
            } else {
                string_map.insert(i, i as u32 - 96);
            }
        }

        for i in split_string.0.chars() {
            let match_found = string_map.get(&i);
            if match_found != None {
                total_score += match_found.unwrap();
                break
            }
        }
    }
    print!("{}",total_score);
}



fn main() {
    //part1();
    
    let sacks = read_file_vec("sacks2.txt").unwrap();
    let chunked_items: Vec<Vec<String>> = sacks
    .chunks(3)
    .map(|x: &[String]| x.to_vec())
    .collect();

    let mut total_score = 0;

    for item in chunked_items {
        let mut string_map0 = HashMap::new();
        let mut string_map1 = HashMap::new();

        for i in item[1].chars() {
            let upper = i.is_uppercase();
            if upper {
                string_map0.insert(i, 26 + (i as u32 - 64));
            } else {
                string_map0.insert(i, i as u32 - 96);
            }
        }

        for i in item[2].chars() {
            let upper = i.is_uppercase();
            if upper {
                string_map1.insert(i, 26 + (i as u32 - 64));
            } else {
                string_map1.insert(i, i as u32 - 96);
            }
        }

        for i in item[0].chars() {
            let match_found = string_map0.get(&i);
            let match_found2 = string_map1.get(&i);
            if match_found != None && match_found2 != None {
                total_score += match_found.unwrap();
                break
            }
        }

    }
    print!("{}",total_score);

}
