use std::io;
mod event;
use event::Event;

fn main() {
    vector_any();
}

fn vector_any() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 10];

    for v in v2 {
        println!("{}", v);
    }
}

fn check_temp_fun() {
    let temp1 = 77.0; // корректная температура
    let temp2 = -500.0; // ниже абсолютного нуля

    let c1 = fahrenheit_to_celsius(temp1);
    let c2 = fahrenheit_to_celsius(temp2);

    check_temp(c1);
    check_temp(c2);
}

fn fahrenheit_to_celsius(f: f64) -> Result<f64, &'static str> {
    if f < -459.67 {
        Err("Температура ниже абсолютного нуля!")
    } else {
        Ok((f - 32.0) * 5.0 / 9.0)
    }
}

fn check_temp(temp: Result<f64, &str>) {
    match temp {
        Ok(c) => println!("Температура в цельсиях: {:.1}°C", c),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn learn_check_age2() {
    let unchecked_age = Some(25);
    if let Some(age) = unchecked_age {
        println!("Возраст: {}", age);
    }
    println!("{}", learn_check_age(unchecked_age));
}

fn learn_check_age(age: Option<u8>) -> &'static str {
    match age {
        Some(25) => "Доступ разрешен",
        Some(_) => "Доступ запрещен",
        None => "Доступ запрещен",
    }
}

fn test1() {
    let title = format!("{}, {}", "Угадай", "число");
    let is_true = true;
    if !is_true {
        println!("true");
        counting(3);
    } else {
        println!("false");
        event_list();
    }

    println!("{title}");
    println!("Введите {}", "число: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("Ваше число : {guess}");
}

fn event_list() {
    let events = vec![
        Event {
            year: 2006,
            desc: "Начал создаваться",
            title: String::from("Title1"),
        },
        Event {
            year: 2009,
            desc: "получил спонсирование в Mozzila",
            title: String::from("Title2"),
        },
        Event {
            year: 2010,
            desc: "был официально представлен на Mozzila Summit 2010",
            title: String::from("Title3"),
        },
        Event {
            year: 2012,
            desc: "выпущена официальная версия Rust 0.1",
            title: String::from("Title4"),
        },
        Event {
            year: 2015,
            desc: "вышла первая стабильная версия 1.0",
            title: String::from("Title5"),
        },
    ];

    for e in events {
        // println!("{:?}", e);
        println!("{}: {}", e.year, e.desc);
        println!("{}", std::mem::size_of_val(e.desc));
    }
}

fn counting(a: i32) {
    let mut a1 = 0;
    while a1 < a {
        println!("{a1}");
        a1 += 1;
    }
}
