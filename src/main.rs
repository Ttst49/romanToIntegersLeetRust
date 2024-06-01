pub fn roman_to_int(s: String) -> i32 {
    let chars:Vec<char> = s.chars().collect();
    let mut total = 0;
    for char in chars.windows(2) {
        println!("{:?}",char);
        if char[0] < char[1] {
            println!("<");
            total-=check_number_equivalent(char[0].to_string());
        }else {
            println!(">");
            total+=check_number_equivalent(char[0].to_string());
        }
    }
    println!("{}",chars.len());
    total
}

pub fn check_number_equivalent(letter:String)->i32{
    match letter.trim() {
        "I"=>1,
        "V"=>5,
        "X"=>10,
        "L"=>50,
        "C"=>100,
        "M"=>1000,
        _ => 0
    }
}

fn main() {
    let number = String::from("XVI");
    roman_to_int(number);
}
