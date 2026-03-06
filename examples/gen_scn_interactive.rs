use std::io::{self, Write};
use traffic_tools_rs::common::converters:: { 
    to_scn_format, 
    to_ascii_codes
 };

fn main() {
    println!("🔧 Генератор SCN формата (интерактивный)");
    println!("Введите строку для конвертации в SCN и ASCII");
    println!("(пустая строка для выхода)");
    println!("----------------------------------------");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() {
            println!("👋 Выход...");
            break;
        }

        println!("  SCN   : {}", to_scn_format(input));
        println!("  ASCII : {:?}", to_ascii_codes(input));
        println!("  длина : {} символов, {} байт", 
                 input.chars().count(), 
                 input.len());
        println!("----------------------------------------");
    }
}