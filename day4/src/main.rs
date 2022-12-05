use std::cmp;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_file_vec(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(filepath)).unwrap();
    let reader = BufReader::new(&file);
    let lines = Ok(reader.lines().filter_map(io::Result::ok).collect());
    lines
}

//

fn main() {
    let jobs = read_file_vec("day4.txt").unwrap();

    let mut total = 0;
    let mut totalOverlap = 0;

    for substring in jobs {
        let ranges: Vec<&str> = substring.split(",").collect();
        let r1: Vec<&str> = ranges[0].split("-").collect();
        let r2: Vec<&str> = ranges[1].split("-").collect();

        let r1_0 : i32 = r1[0].parse().expect("Failed to parse integer");
        let r1_1 : i32 = r1[1].parse().expect("Failed to parse integer");

        let r2_0 : i32 = r2[0].parse().expect("Failed to parse integer");
        let r2_1 : i32 = r2[1].parse().expect("Failed to parse integer");

        let range1 = (r1_0)..(r1_1);
        let range2 = (r2_0)..(r2_1);

        if (r2_0 >= r1_0 && r2_0 <= r1_1) && (r2_1 >= r1_0 && r2_1 <= r1_1) || (r1_0 >= r2_0 && r1_0 <= r2_1) && (r1_1 >= r2_0 && r1_1 <= r2_1) {
            total += 1;
        }
        
        if cmp::max(r1_0, r2_0) <= cmp::min(r1_1, r2_1) {
            totalOverlap += 1;
        } 
    }

    println!("{}", total);
    println!("{}", totalOverlap);

}
