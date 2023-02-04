# Password Generator with Rust

A script to create random passwords.

## Dependencies (Cargo.toml)

```
[dependencies]
rand = "0.8.5"
```

## Code

```
use rand::prelude::*;

fn main() {
    let password1 = generate_password(false, false, false, 8);
    let password2 = generate_password(true, true, false, 16);
    let password3 = generate_password(true, true, true, 24);
    println!("Password: {}", password1);
    println!("Password: {}", password2);
    println!("Password: {}", password3);
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

    let mut rng = thread_rng();
    (0..lenght).map(|_| *possible_chars.choose(&mut rng).unwrap()).collect()
}
```

## Output

```
password_generator_rust on  main [!] is 📦 v0.1.0 via 🦀 v1.65.0 
❯ cargo run
    Blocking waiting for file lock on build directory
   Compiling password_generator_rust v0.1.0 (/home/deimos/Software dev/Rust/Project/password_generator_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/password_generator_rust`
Password: nearzjyz
Password: ajrnosOMGHSMFGt4
Password: KzVRQ!qg8eiUizb82zpo!w85
```