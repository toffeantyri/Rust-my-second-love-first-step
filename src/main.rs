use rand::Rng;
use std::{f64::consts, io, vec};
mod event;
mod summary;
use event::Event;
mod csvdata;

///Тестовая программа для изучения синтаксиса языка Rust (cargo doc --open)
fn main() {
    on_off_features();
}

fn on_off_features() {
    #[cfg(feature = "no_std")]
    {
        println!("FEATURE: NO STD - on ")
    }

    #[cfg(not(feature = "no_std"))]
    {
        println!("FEATURE: NO STD - off ")
    }
}

fn print_os_arch() {
    let pointer_width_bits = std::mem::size_of::<*const ()>() * 8;
    println!("Разрядность указателя {} бит", pointer_width_bits);
    println!("Целевая архитектура {}", std::env::consts::ARCH);
    println!("Целевая платформа {}", std::env::consts::OS);
}

fn for_by_iter() {
    let vector = vec![1, 2, 3, 4, 5];

    for v in vector {
        println!("item: {}", v);
    }
    //println!("{:?}", vector); //будет ошибка - vector уже недоступен
    //  тк его потребил (move) цикл for
    let vector1 = vec![1, 2, 3, 4, 5];

    for v in &vector1 {
        println!("item: {}", v);
    }
    println!("{:?}", vector1);
    //все ок - передаем только ссылку на вектор

    let mut vector2 = vec![1, 2, 3, 4, 5];

    for v in &mut vector2 {
        //for v in vector2.iter_mut() - аналог {
        *v *= *v;
        println!("item: {}", v);
    }
    println!("{:?}", vector2); // этот вариант если надо изменить элементы коллекции
}

fn random_fn() {
    for _i in 0..100 {
        let r = rand::thread_rng().gen_range(0..=10);

        println!("{}", r);
    }
}

fn any_fn() {
    let mut num_str0 = String::new();
    io::stdin().read_line(&mut num_str0).expect("error");
    match num_str0.trim().parse::<i32>() {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }

    let mut num_str = String::new();
    if io::stdin().read_line(&mut num_str).is_ok() {
        let num = num_str.trim().parse::<i32>();
        println!("{:02}", num.unwrap())
    }

    let mut num_str2 = String::new();
    io::stdin()
        .read_line(&mut num_str2)
        .expect("Error of input");
    println!("{}", num_str2);
}

fn kortezh_tuples() {
    let _tuple: (i32, f64, String) = (12, 34.6, String::from("123"));
    let pupil = ("Denis", 11);
    let (name, grade) = pupil;
    println!("Name is {}, grade is {}", name, grade);
    let _name2 = pupil.0;
    let _grade2 = pupil.1;
}

fn array_learn() {
    let _empty_array = [0; 10]; //10 x 0
    let _int_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let _int_array: [i16; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
}

fn kvadratnoe_uravnenie() {
    loop {
        //ax^2+bx+c=0
        let mut a_str = String::new();
        let mut b_str = String::new();
        let mut c_str = String::new();
        println!("Вычисление корней квадратного уравнения \n ax^2 + bx + c = 0");
        println!("Введите a");

        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {}
            Err(e) => {
                println!("Input error {}", e)
            }
        }

        println!("Введите b");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {}
            Err(e) => println!("Input error  {}", e),
        }

        println!("Введите c");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {}
            Err(e) => println!("Input error  {}", e),
        }

        let a: f64 = a_str.trim().parse().unwrap();
        let b: f64 = b_str.trim().parse().unwrap();
        let c: f64 = c_str.trim().parse().unwrap();

        println!("{}x^2 + {}x + {} = 0", a, b, c);

        let d: f64 = (b * b) - 4f64 * a * c;
        if d > 0f64 {
            let x1 = ((-b) + d.sqrt()) / 2f64 * a;
            let x2 = ((-b) - d.sqrt()) / 2f64 * a;
            println!(
                "Решено, есть 2 корня. D = {}, \n x1 = {}, \n x2 = {}",
                d, x1, x2
            );
        } else if d == 0.0 {
            let x = (-b) / 2.0 * a;
            println!("Решено, есть 1 корень. D = {}, \n x1 = {}", d, x);
        } else {
            println!("Решено. Корней нет. D = {}", d,);
        }
    }
}

fn io_learning() {
    let mut name = String::new();
    println!("Input your name: ");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Hello {}", name)
        }
        Err(e) => {
            println!("Error input {}", e)
        }
    }
}

fn str_comp_example() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);
}

fn vector_any() {
    let _v1 = [1, 2, 3, 4, 5];
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
    println!("Введите число: ");

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

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }
}
