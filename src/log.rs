use crate::dto::Trade;
use crate::blockchain::get_current_blockchain_symbols;
use prettytable::{Table, Row, Cell, Attr, color};

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
                if zone == "Z1" || zone == "Z7" {
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
        Cell::new("BTC"),
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

    let active_symbols = get_current_blockchain_symbols();

    for trade in trades {
        let mut symbol_cell = Cell::new(&trade.symbol);
        if active_symbols.contains(&trade.symbol) {
            symbol_cell = symbol_cell.with_style(Attr::ForegroundColor(color::YELLOW));
        }

        table.add_row(Row::new(vec![
            symbol_cell,
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
