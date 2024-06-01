pub fn roman_to_int(s: String) -> i32 {
    let s = s
        .replace("IV", "IIII")
        .replace("IX", "VIIII")
        .replace("XL", "XXXX")
        .replace("XC", "LXXXX")
        .replace("CD", "CCCC")
        .replace("CM", "DCCCC");

    let total = s.chars().map(|c| {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }).sum();
    total
}


fn main() {
    let number = String::from("II");
    roman_to_int(number);
}
