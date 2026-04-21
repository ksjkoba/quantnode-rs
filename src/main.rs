struct PriceUpdate {
    symbol: String,
    price: f64,
    volume: u64,
}

fn process_trade(update: PriceUpdate) {
    println!(
        "[*] Processing trade for {}: Price  | Vol {}", 
        update.symbol, update.price, update.volume
    );
}

fn main() {
    println!("🚀 QuantNode-RS initialized. Listening for market data...");

    // Simulating an incoming data stream
    let trades = vec![
        PriceUpdate { symbol: String::from("BTC"), price: 65000.50, volume: 10 },
        PriceUpdate { symbol: String::from("ETH"), price: 3500.25, volume: 50 },
    ];

    for trade in trades {
        process_trade(trade);
    }

    println!("[+] Execution loop complete.");
}
