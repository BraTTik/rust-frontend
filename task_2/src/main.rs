fn fizz_buzz(n: i32) {
    if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz")
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
}

fn print_format() {
    let name = "Alice";
    let age = 25;
    let height = 1.75;
    let numbers = [1, 2, 3, 4, 5];

    println!("1. Привет, {}! Тебе {} лет.", name, age);
    println!("2. Имя {1}, возраст {0}, рост {2} м.", name, age, height);
    println!("3. Имя: {name}, возраст: {age}, рост: {height} м.");
    println!("4. Число π: {:.3}", std::f64::consts::PI);
    println!("5. Двоичное число: {:b}, Шестнадцатеричное число: {:X}", 255, 255);
    println!("6. |{:<10}|{:^10}|{:>10}|", "левый", "центр", "правый");
    println!("7. Отладка: {:?}", numbers);
    println!("8. Красиво: {:#?}", numbers);
    println!("9. Заполнение нулями: {:05}", 42);
    println!("10. Научная: {:e}", 1234567.89);
}

fn date_format(year: i32, month: i32, day: i32) {
    println!();
    println!("Дата: ");
    println!("📅 {:04}-{:02}-{:02}", year, month, day);
    println!();
}

fn money_format(amount: f32) {

    println!();
    println!("💵 {:.2}", amount);
    println!();
}

fn hex_color(r: u8, g: u8, b: u8) {
    println!();
    println!("HEX: #{:02X}{:02X}{:02X}", r, g, b);
    println!();
}

fn table_format(name: &str, age: u8, score: f32) {
    println!();
    println!("|{:^10}|{:^10}|{:^10}|", "Имя", "Возраст", "Оценка");
    println!("|{:^10}|{:^10}|{:^10.2}|", name, age, score);
    println!();
}

fn main() {
    let name = "Alice";
    let age = 25;

    fizz_buzz(6);
    fizz_buzz(10);
    fizz_buzz(15);
    fizz_buzz(19);

    print_format();

    let year = 2025;
    let month = 1;
    let day = 15;

    date_format(year, month, day);

    let amount = 142.9765;
    money_format(amount);

    let r = 255;
    let g = 128;
    let b = 0;
    hex_color(r, g, b);

    table_format(name, age, 95.543)
}
