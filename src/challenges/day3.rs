use std::fs::File;
use std::io::*;
use colored::*;
use std::iter::Iterator;

fn parse_file(filename: String) -> Vec<i16> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<i16> = Vec::new();

    for line in reader.lines() {
        let intval = i16::from_str_radix(&line.unwrap(), 2).unwrap();
        result.push(intval);
    }

    return result;
}

/// Caclulate the power consumption

/// Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.
/// The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.
/// The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.
/// So, the gamma rate is the binary number 10110, or 22 in decimal.
/// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
fn part_1(measurements: &Vec<i16>) {

    let mut mask: i16 = 0b100000000000;
    let mut gamma: i16 = 0;

    for i in (0..12).rev() {
        gamma =
            gamma | ((
                (measurements.iter().enumerate().filter(|(_index, value)| *value & mask == mask).count() >= 
                (measurements.len()/2)) as i16) << i);
        mask = mask >> 1;
    }
    println!("{}: {1:012b} ({1})", "Gamma".blue(), gamma);
    let epsilon = gamma^0b00111111111111;
    println!("{}: {1:012b} ({1})", "Epsilon".blue(), epsilon);
    let power_consumption: i64 = (gamma as i64) * (epsilon as i64);
    println!("{}: {1:012b} ({1})", "Power consumptoion".green(), power_consumption);

}

/// --- Part Two ---
/// Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.
/// 
/// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:
/// - Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
/// - If you only have one number left, stop; this is the rating value for which you are searching.
/// - Otherwise, repeat the process, considering the next bit to the right.
/// 
/// The bit criteria depends on which type of rating value you want to find:
/// - To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
/// - To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
///
/// For example, to determine the oxygen generator rating value using the same example diagnostic report from above:
/// - Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
/// - Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
/// - In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
/// - In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
/// - In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
/// - As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
/// 
/// Then, to determine the CO2 scrubber rating value from the same example above:
/// - Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
/// - Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
/// - In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
/// - As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
/// - Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.
/// 
/// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)
fn part_2(measurements: &Vec<i16>) {

    // First calculate the oxygen
    let mut mcmask: i16 = (find_most_common(measurements, 12) as i16) << 12;
    let mut subset: Vec<i16> = measurements.clone();
    println!("mcmask: ");

    for i in (0..12).rev() {
        subset.retain(|&x| x & mcmask == mcmask);
        if subset.len() <=1 { break; }
        mcmask = (find_most_common(measurements, i) as i16) << i;
        println!("mcmask: {:b}, subset.len(): {}", mcmask, subset.len());
        if mcmask <= 0 {break;}
    }

    subset.iter().for_each(|x| println!("{}: {}", "Oxygen".blue(), x));

//   let mut xs = vec![1, 2, 3];
//    let some_x = 2;
//    xs.retain(|&x| x != some_x);
//    println!("{:?}", xs); // prints [1, 3]
    

}

/// From a list of bits return the most common bit.
/// If there are equally common numbers based on the request bit then the request bit is returend
/// 
/// The most common is determined by summing all 0 or 1 and see which occures the most
/// @param bitlist: Vector containing all the bits to search in
/// @param index: get the position in the bitvalue (15..0) 
/// @return: 1 of the most common on the index = 1; else 0 
fn find_most_common(bitlist: &Vec<i16>, index: i16) -> i8 {

    let mask = 0b1 << index;
    
    (bitlist.iter().enumerate().filter(|(_index, value)| *value & mask == mask).count() >= 
    bitlist.len()/2) as i8
}

pub fn run() {
    let filename: String = "data/example_day_3".to_string();
    //let filename: String = "data/day3".to_string();
    let measurements = parse_file(filename);

    part_1(&measurements);
    part_2(&measurements);
}
