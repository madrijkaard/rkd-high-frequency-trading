use crate::dto::Trade;

pub fn log_current_zone(trade: &Trade) {
    
    print!("\x1B[2J\x1B[1;1H");

    let current_price = parse(&trade.current_price);
    let zones = vec![
        ("Z1", parse(&trade.zone_1)),
        ("Z2", parse(&trade.zone_2)),
        ("Z3", parse(&trade.zone_3)),
        ("Z4", parse(&trade.zone_4)),
        ("Z5", parse(&trade.zone_5)),
        ("Z6", parse(&trade.zone_6)),
        ("Z7", parse(&trade.zone_7)),
        ("Z8", f64::MAX),
    ];

    println!("");

    println!("+------------------------+");

    for i in (0..zones.len()).rev() {
        let label = zones[i].0;
        let lower = if i == 0 { 0.0 } else { zones[i - 1].1 };
        let upper = zones[i].1;

        let is_current_zone = current_price > lower && current_price <= upper
            || (label == "Z1" && current_price <= lower);

        if is_current_zone {
            println!("\x1b[44;97m| {:^22} |\x1b[0m", format!("{:.2}", current_price));
        } else {
            println!("| {:^22} |", label);
        }

        if i != 0 {
            println!("|------------------------|");
        }
    }

    println!("+------------------------+");
    println!(
        "\n[{}] - Preco atual: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        trade.current_price
    );
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}
