use crate::dto::Trade;

pub fn log_current_zone(trade: &Trade) {

    let current_price = parse(&trade.current_price);
    let z7 = parse(&trade.zone_7);
    let z6 = parse(&trade.zone_6);
    let z5 = parse(&trade.zone_5);
    let z4 = parse(&trade.zone_4);
    let z3 = parse(&trade.zone_3);
    let z2 = parse(&trade.zone_2);
    let z1 = parse(&trade.zone_1);

    let (zona_a, zona_b) = if current_price > z7 {
        ("MAX", "Z7")
    } else if current_price > z6 && current_price <= z7 {
        ("Z7", "Z6")
    } else if current_price > z5 && current_price <= z6 {
        ("Z6", "Z5")
    } else if current_price > z4 && current_price <= z5 {
        ("Z5", "Z4")
    } else if current_price > z3 && current_price <= z4 {
        ("Z4", "Z3")
    } else if current_price > z2 && current_price <= z3 {
        ("Z3", "Z2")
    } else if current_price > z1 && current_price <= z2 {
        ("Z2", "Z1")
    } else {
        ("Z1", "MIN")
    };

    println!(
        "[{}] - {} is between {} and {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        trade.current_price,
        zona_a,
        zona_b
    );
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}
