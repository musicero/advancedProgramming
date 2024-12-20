use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};

fn main() {
    let txs_reader = BufReader::new(File::open("txs.txt").expect("Open txs.txt"));
    let mut fees_writer = BufWriter::new(File::create("fees.txt").expect("Create fees.txt"));
    let mut earnings_writer =
        BufWriter::new(File::create("earnings.txt").expect("Create earnings.txt"));

    let mut txs_sum: f64 = 0.;
    let mut fees_sum: f64 = 0.;
    let mut earnings_sum: f64 = 0.;

    for line in txs_reader.lines() {
        let txs = line
            .expect("failed to read line")
            .parse::<f64>()
            .expect("failed to parse float");
        let (fee, earning) = (txs * 0.3, txs * 0.7);
        writeln!(fees_writer, "{:.2}", fee).expect("Write fee to file");
        writeln!(earnings_writer, "{:.2}", earning).expect("Write earning to file");
        txs_sum += txs;
        fees_sum += txs * 0.3;
        earnings_sum += txs * 0.7;
    }

    let (total_fees, total_earnings) = (txs_sum * 0.3, txs_sum * 0.7);

    println!("TXS_SUM: {:.2}", txs_sum);
    println!("FEES_SUM:   {:.2}", fees_sum);
    println!("FEES_TOTAL: {:.2}", total_fees);
    println!("EARNINGS_SUM:   {:.2}", earnings_sum);
    println!("EARNINGS_TOTAL: {:.2}", total_earnings);
    println!("EARNINGS_FEES_SUM: {:.2}", total_fees + total_earnings);
}
