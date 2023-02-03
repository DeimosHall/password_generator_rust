fn main() {
    let hello = "Hello World";
    println!("{}", hello);
    generate_password(false, false, true, 16);
}

fn generate_password(hasNumbers: bool, hasCapLetters: bool, hasSymbols: bool, lenght: u8) -> String {
    let symbols_list = vec!['!', '#', '$'];
    let numbers_list: Vec<char> = (b'0'..=b'9').map(|x| x as char).collect();
    let cap_letters_list: Vec<char> = (b'A'..=b'Z').map(|x| x as char).collect();
    let mut possible_chars: Vec<char> = (b'a'..=b'z').map(|x| x as char).collect();
    println!("{:?}", numbers_list);
    String::from("")
}
