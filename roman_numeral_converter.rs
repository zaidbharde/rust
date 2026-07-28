fn int_to_roman(mut num: u32) -> String {
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut result = String::new();

    for (i, &value) in values.iter().enumerate() {
        while num >= value {
            result.push_str(symbols[i]);
            num -= value;
        }
    }
    result
}

fn roman_to_int(s: &str) -> u32 {
    let values = |c: char| match c {
        'I' => 1, 'V' => 5, 'X' => 10, 'L' => 50,
        'C' => 100, 'D' => 500, 'M' => 1000,
        _ => 0,
    };
    let chars: Vec<char> = s.chars().collect();
    let mut total = 0;
    for i in 0..chars.len() {
        let curr = values(chars[i]);
        if i + 1 < chars.len() && curr < values(chars[i + 1]) {
            total -= curr;
        } else {
            total += curr;
        }
    }
    total
}

fn main() {
    let num = 1994;
    let roman = int_to_roman(num);
    println!("{} -> {}", num, roman);
    println!("{} -> {}", roman, roman_to_int(&roman));
}
