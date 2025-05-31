use crate::blockchain::get_current_blockchain_symbols;
use crate::config::Settings;
use crate::dto::{Bias, Trade};
use prettytable::{color, Attr, Cell, Row, Table};

pub fn monitor_cryptos(trades: &[Trade], settings: &Settings) {
    fn parse(value: &str) -> f64 {
        value.parse::<f64>().unwrap_or(0.0)
    }

    fn max_min(values: &[f64]) -> (f64, f64) {
        let max = values.iter().cloned().fold(f64::MIN, f64::max);
        let min = values.iter().cloned().fold(f64::MAX, f64::min);
        (max, min)
    }

    fn find_zone_index(trade: &Trade) -> Option<usize> {
        let price = parse(&trade.current_price);
        let zones = vec![
            parse(&trade.zone_1),
            parse(&trade.zone_2),
            parse(&trade.zone_3),
            parse(&trade.zone_4),
            parse(&trade.zone_5),
            parse(&trade.zone_6),
            parse(&trade.zone_7),
            f64::MAX,
        ];

        for i in 0..zones.len() {
            let lower = if i == 0 { 0.0 } else { zones[i - 1] };
            let upper = zones[i];
            if price > lower && price <= upper || (i == 0 && price <= lower) {
                return Some(i);
            }
        }

        None
    }

    fn zone_label_cell(index: Option<usize>, bias: &Bias) -> Cell {
        match index {
            Some(i) => {
                let zone_str = format!("Z{}", i + 1);
                match (i, bias) {
                    (0, Bias::Bullish) | (6, Bias::Bullish) => {
                        Cell::new(&zone_str).with_style(Attr::ForegroundColor(color::BLUE))
                    }
                    (1, Bias::Bearish) | (7, Bias::Bearish) => {
                        Cell::new(&zone_str).with_style(Attr::ForegroundColor(color::BLUE))
                    }
                    _ => Cell::new(&zone_str),
                }
            }
            None => Cell::new("-"),
        }
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

    fn highlight_f64_cell(value: &str, max: f64, min: f64) -> Cell {
        let val = parse(value);
        let mut cell = Cell::new(&format!("{:.2}", val));
        if val == max {
            cell = cell.with_style(Attr::ForegroundColor(color::GREEN));
        } else if val == min {
            cell = cell.with_style(Attr::ForegroundColor(color::RED));
        }
        cell
    }

    fn highlight_val_cell(val: f64, max: f64, min: f64) -> Cell {
        let mut cell = Cell::new(&format!("{:.2}", val));
        if val == max {
            cell = cell.with_style(Attr::ForegroundColor(color::GREEN));
        } else if val == min {
            cell = cell.with_style(Attr::ForegroundColor(color::RED));
        }
        cell
    }

    fn calc_log_ampl(min: f64, max: f64) -> f64 {
        if min <= 0.0 || max <= 0.0 || min >= max {
            return 0.0;
        }
        (max.ln() - min.ln()) * 100.0
    }

    fn calc_log_position(price: f64, min: f64, max: f64) -> f64 {
        if min <= 0.0 || max <= 0.0 || price <= 0.0 || min >= max {
            return 0.0;
        }
        ((price.ln() - min.ln()) / (max.ln() - min.ln())) * 100.0
    }

    print!("\x1B[2J\x1B[1;1H");
    let now = chrono::Local::now();
    println!("[{}] - Criptos monitoradas:", now.format("%Y-%m-%d %H:%M:%S"));

    let mut table = Table::new();
    let show_details = settings.show_details_monitor;
    if show_details {
        table.add_row(Row::new(vec![
            Cell::new("Symbol"),
            Cell::new("Zone"),
            Cell::new("24h"),
            Cell::new("BTC"),
            Cell::new("MA200"),
            Cell::new("Ampl"),
            Cell::new("Pos%"),
            Cell::new("Volume"),
            Cell::new("Quote Volume"),
            Cell::new("Trades"),
            Cell::new("Taker Base"),
            Cell::new("Taker Quote"),
        ]));
    } else {
        table.add_row(Row::new(vec![
            Cell::new("Symbol"),
            Cell::new("Zone"),
            Cell::new("24h"),
            Cell::new("BTC"),
            Cell::new("MA200"),
        ]));
    }

    let mut perf_vals = vec![];
    let mut btc_vals = vec![];
    let mut amp_vals = vec![];
    let mut log_ampls = vec![];
    let mut log_positions = vec![];
    let mut volumes = vec![];
    let mut quote_volumes = vec![];
    let mut trades_counts = vec![];
    let mut taker_base_volumes = vec![];
    let mut taker_quote_volumes = vec![];
    let mut zone_counts = [0usize; 8];

    for t in trades {
        perf_vals.push(parse(&t.performance_24));
        btc_vals.push(parse(&t.performance_btc_24));
        amp_vals.push(parse(&t.amplitude_ma_200));

        let min = parse(&t.zone_min);
        let max = parse(&t.zone_max);
        log_ampls.push(calc_log_ampl(min, max));
        log_positions.push(calc_log_position(parse(&t.current_price), min, max));

        volumes.push(parse(&t.volume));
        quote_volumes.push(parse(&t.quote_asset_volume));
        trades_counts.push(parse(&t.number_of_trades));
        taker_base_volumes.push(parse(&t.taker_buy_base_asset_volume));
        taker_quote_volumes.push(parse(&t.taker_buy_quote_asset_volume));
    }

    let (max_perf, min_perf) = max_min(&perf_vals);
    let (max_btc_perf, min_btc_perf) = max_min(&btc_vals);
    let (max_amplitude, min_amplitude) = max_min(&amp_vals);
    let (max_log_ampl, min_log_ampl) = max_min(&log_ampls);
    let (max_pos, min_pos) = max_min(&log_positions);
    let (max_vol, min_vol) = max_min(&volumes);
    let (max_quote_vol, min_quote_vol) = max_min(&quote_volumes);
    let (max_trades, min_trades) = max_min(&trades_counts);
    let (max_taker_base, min_taker_base) = max_min(&taker_base_volumes);
    let (max_taker_quote, min_taker_quote) = max_min(&taker_quote_volumes);

    let active_symbols = get_current_blockchain_symbols();

    for (idx, trade) in trades.iter().enumerate() {
        let mut symbol_cell = Cell::new(&trade.symbol);
        if active_symbols.contains(&trade.symbol) {
            symbol_cell = symbol_cell.with_style(Attr::ForegroundColor(color::YELLOW));
        }

        let zone_index = find_zone_index(trade);
        if let Some(z) = zone_index {
            zone_counts[z] += 1;
        }

        let mut row = vec![
            symbol_cell,
            zone_label_cell(zone_index, &trade.bias),
            highlight_cell(&trade.performance_24, max_perf, min_perf),
            highlight_cell(&trade.performance_btc_24, max_btc_perf, min_btc_perf),
            highlight_cell(&trade.amplitude_ma_200, max_amplitude, min_amplitude),
        ];

        if show_details {
            row.push(highlight_val_cell(log_ampls[idx], max_log_ampl, min_log_ampl));
            row.push(highlight_val_cell(log_positions[idx], max_pos, min_pos));
            row.push(highlight_f64_cell(&trade.volume, max_vol, min_vol));
            row.push(highlight_f64_cell(&trade.quote_asset_volume, max_quote_vol, min_quote_vol));
            row.push(highlight_f64_cell(&trade.number_of_trades, max_trades, min_trades));
            row.push(highlight_f64_cell(&trade.taker_buy_base_asset_volume, max_taker_base, min_taker_base));
            row.push(highlight_f64_cell(&trade.taker_buy_quote_asset_volume, max_taker_quote, min_taker_quote));
        }

        table.add_row(Row::new(row));
    }

    table.printstd();

    println!(
        "\nDistribuicao por zona: {}",
        zone_counts
            .iter()
            .enumerate()
            .map(|(i, count)| format!("Z{}: {}", i + 1, count))
            .collect::<Vec<_>>()
            .join(" | ")
    );
}
