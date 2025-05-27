use crate::dto::Trade;
use prettytable::{Table, Row, Cell, Attr, color};

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

    println!();
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

pub fn log_spied_cryptos(trades: &[Trade]) {
    fn find_zone_label(trade: &Trade) -> Cell {
        let price = parse(&trade.current_price);
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

        for i in 0..zones.len() {
            let lower = if i == 0 { 0.0 } else { zones[i - 1].1 };
            let upper = zones[i].1;
            if price > lower && price <= upper || (i == 0 && price <= lower) {
                let zone = zones[i].0;
                let mut cell = Cell::new(zone);
                if zone == "Z1" || zone == "Z8" {
                    cell = cell.with_style(Attr::ForegroundColor(color::BLUE));
                }
                return cell;
            }
        }

        Cell::new("-")
    }

    fn highlight_cell(value: &str, max: f64, min: f64) -> Cell {
        let val = parse(value);
        let mut cell = Cell::new(value);
        if val == max {
            cell = cell.with_style(Attr::ForegroundColor(color::GREEN));
        } else if val == min {
            cell = cell.with_style(Attr::ForegroundColor(color::RED));
        }
        cell
    }

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "[{}] - Criptos monitoradas:",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Symbol"),
        Cell::new("Zone"),
        Cell::new("24h"),
        Cell::new("BTC 24h"),
        Cell::new("MA200"),
    ]));

    let mut perf_vals = vec![];
    let mut btc_vals = vec![];
    let mut amp_vals = vec![];

    for t in trades {
        perf_vals.push(parse(&t.performance_24));
        btc_vals.push(parse(&t.performance_btc_24));
        amp_vals.push(parse(&t.amplitude_ma_200));
    }

    let (max_perf, min_perf) = max_min(&perf_vals);
    let (max_btc_perf, min_btc_perf) = max_min(&btc_vals);
    let (max_amplitude, min_amplitude) = max_min(&amp_vals);

    for trade in trades {
        table.add_row(Row::new(vec![
            Cell::new(&trade.symbol),
            find_zone_label(trade),
            highlight_cell(&trade.performance_24, max_perf, min_perf),
            highlight_cell(&trade.performance_btc_24, max_btc_perf, min_btc_perf),
            highlight_cell(&trade.amplitude_ma_200, max_amplitude, min_amplitude),
        ]));
    }

    table.printstd();
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

fn max_min(values: &[f64]) -> (f64, f64) {
    let max = values.iter().cloned().fold(f64::MIN, f64::max);
    let min = values.iter().cloned().fold(f64::MAX, f64::min);
    (max, min)
}
