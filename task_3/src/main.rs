fn calculate(a: i32, b: i32, operator: &str) -> i32 {
    if operator == "+" {
        i32::saturating_add(a, b)
    } else if operator == "-" {
        i32::saturating_sub(a, b)
    } else if operator == "*" {
        i32::saturating_mul(a, b)
    } else {
        0
    }
}

fn format_message(name: &str, score: u32, level: u32) -> String {
    format!("Привет, {name}! Твой текущий счет: {score}, уровень: {level}")
}

fn build_greeting(name: &str, suffix: &str) -> String {
    let mut str = name.to_string();
    str.push(',');
    str.push(' ');
    str.push_str(suffix);

    str
}

fn main() {
    println!("{}", calculate(1, 2, "+"));
    println!("{}", calculate(9, 3, "-"));
    println!("{}", calculate(3, 3, "*"));
    println!("{}", calculate(3, 3, "/"));

    println!("{}", format_message("Ivan", 100, 1));

    println!("{}", build_greeting("Ivan", "привет"));
}
