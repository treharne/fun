// Aim: 
// Find the integer fraction with the lowest denominator 
// between two given fractions: a/b and c/d

use std::io;

fn main() {
    let [a, b, c, d] = get_input_vals();
    let [numerator, denominator] = simplest_intermediate_fraction(a, b, c, d);
    println!("Fraction is {}/{}", numerator, denominator)
}

fn simplest_intermediate_fraction(a: u32, b: u32, c: u32, d: u32) -> [u32; 2] {
    let x = frac(a, b);
    let y = frac(c, d);

    if x >= y {
        panic!("x must be less than y")
    }

    for denominator in 1.. {
        let x_equiv_numerator = x * denominator as f32;
        let y_equiv_numerator = y * denominator as f32;

        let numerator = next_integer(x_equiv_numerator);

        if (numerator as f32) < y_equiv_numerator {
            return [numerator, denominator]
        };
    }

    panic!("Could not find an intermediate fraction")
}

fn next_integer(f: f32) -> u32 {
    f as u32 + 1
}

fn frac(numerator: u32, denominator: u32) -> f32 {
    numerator as f32 / denominator as f32
}


fn get_input_vals() ->  [u32; 4] {
    println!("First let's define the two fractions. a/b and c/d");
    println!("Enter values for a, b, c and d separated by spaces:");
    
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    
    let vals = input.split_whitespace().map(|val| parse_int_from_str(val)).collect::<Vec<u32>>();
    
    if vals.len() != 4 {
        panic!("Must provide 4 values");
    }
    
    [vals[0], vals[1], vals[2], vals[3]]
}

fn parse_int_from_str(input_str: &str) -> u32 {
    input_str.trim().parse().expect("Must be integers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(simplest_intermediate_fraction(2, 5, 4, 5), [1, 2]);
        assert_eq!(simplest_intermediate_fraction(1, 1, 2, 1), [3, 2]);
        assert_eq!(simplest_intermediate_fraction(5, 13, 7, 18), [12, 31]);
        assert_eq!(simplest_intermediate_fraction(12, 31, 7, 18), [19, 49]);
    }
}
