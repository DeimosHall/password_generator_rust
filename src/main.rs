fn main() {
    let hello = "Hello World";
    println!("{}", hello);
    generate_password(false, false, true, 16);
}

fn generate_password(has_numbers: bool, has_cap_letters: bool, has_symbols: bool, lenght: u8) -> String {
    let symbols_list = vec!['!', '#', '$'];
    let numbers_list: Vec<char> = (b'0'..=b'9').map(|x| x as char).collect();
    let cap_letters_list: Vec<char> = (b'A'..=b'Z').map(|x| x as char).collect();
    let mut possible_chars: Vec<char> = (b'a'..=b'z').map(|x| x as char).collect();

    if has_numbers {
        possible_chars.extend(numbers_list.iter().cloned());
    }
    if has_cap_letters {
        possible_chars.extend(cap_letters_list.iter().cloned());
    }
    if has_symbols {
        possible_chars.extend(symbols_list.iter().cloned());
    }

    String::from("")
}
