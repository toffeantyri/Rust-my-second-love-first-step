pub fn csvdata() {
    let csv_data = "\
Название, Диаметр (км), Масса (Кг)
Меркурий, 4879.4, 3.302*10^23
Венера, 12103.6, 4.869*10^24
Земля, 12756.3, 5.974*10^24
Марс, 6794.4, 6.419*10^23
        ";

    let records = csv_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            //eprintln!("Отладка: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {} км, {} кг", name, length, fields[2]);
        }
    }
}
