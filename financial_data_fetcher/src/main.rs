use serde::Deserialize;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::thread;
use std::time::Duration;

// enum used for error handling the api calls
#[derive(Debug)]
enum PricingApiResult<T> {
    Success(T),
    ApiError(String),
    NetworkError(String),
}

// structs use to deserialize the information from the endpoint
#[derive(Debug, Deserialize)]
struct YahooChartResponse {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Meta {
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
}

// trait shared by the bitcoin, etherum and sp500 structs
trait Pricing {
    fn fetch_price(&self) -> PricingApiResult<f64>;
    fn save_to_file(&self, price: f64);
    fn name(&self) -> &str;
}

struct Bitcoin {
    price: f64,
}

struct Ethereum {
    price: f64,
}

struct SP500 {
    price: f64,
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> PricingApiResult<f64> {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/BTC-USD";
        let response = ureq::get(url).call();
        match response {
            Ok(resp) => {
                if resp.status() == 200 {
                    match resp.into_json::<YahooChartResponse>() {
                        Ok(parsed) => {
                            if let Some(first) = parsed.chart.result.first() {
                                PricingApiResult::Success(first.meta.regular_market_price)
                            } else {
                                PricingApiResult::ApiError("No result in chart".to_string())
                            }
                        },
                        Err(e) => PricingApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    PricingApiResult::ApiError(format!("HTTP error: {}", resp.status()))
                }
            },
            Err(e) => PricingApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }
    fn name(&self) -> &str {
        "Bitcoin"
    }
    fn save_to_file(&self, price_val: f64) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("bitcoin_prices") {
            let _ = writeln!(file, "{}: ${}", self.name(), price_val);
        }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> PricingApiResult<f64> {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/ETH-USD";
        let response = ureq::get(url).call();
        match response {
            Ok(resp) => {
                if resp.status() == 200 {
                    match resp.into_json::<YahooChartResponse>() {
                        Ok(parsed) => {
                            if let Some(first) = parsed.chart.result.first() {
                                PricingApiResult::Success(first.meta.regular_market_price)
                            } else {
                                PricingApiResult::ApiError("No result in chart".to_string())
                            }
                        },
                        Err(e) => PricingApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    PricingApiResult::ApiError(format!("HTTP error: {}", resp.status()))
                }
            },
            Err(e) => PricingApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }
    fn name(&self) -> &str {
        "Ethereum"
    }
    fn save_to_file(&self, price_val: f64) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("etherum_prices") {
            let _ = writeln!(file, "{}: ${}", self.name(), price_val);
        }
    }
}


impl Pricing for SP500 {
    fn fetch_price(&self) -> PricingApiResult<f64> {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        let response = ureq::get(url).call();
        match response {
            Ok(resp) => {
                if resp.status() == 200 {
                    match resp.into_json::<YahooChartResponse>() {
                        Ok(parsed) => {
                            if let Some(first) = parsed.chart.result.first() {
                                PricingApiResult::Success(first.meta.regular_market_price)
                            } else {
                                PricingApiResult::ApiError("No result in chart".to_string())
                            }
                        },
                        Err(e) => PricingApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    PricingApiResult::ApiError(format!("HTTP error: {}", resp.status()))
                }
            },
            Err(e) => PricingApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }
    fn name(&self) -> &str {
        "S&P 500"
    }
    fn save_to_file(&self, price_val: f64) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("sp500_prices") {
            let _ = writeln!(file, "{}: ${}", self.name(), price_val);
        }
    }
}


fn main() {
    let mut prices: Vec<Box<dyn Pricing>> = Vec::new();
    prices.push(Box::new(Bitcoin { price: 0.0 }));
    prices.push(Box::new(Ethereum { price: 0.0 }));
    prices.push(Box::new(SP500 { price: 0.0 }));

    loop {
        for price in &prices {
            match price.fetch_price() {
                PricingApiResult::Success(val) => {
                    println!("{} price: ${}", price.name(), val);
                    price.save_to_file(val);
                },
                PricingApiResult::ApiError(e) => println!("{} API Error: {}", price.name(), e),
                PricingApiResult::NetworkError(e) => println!("{} Network Error: {}", price.name(), e),
            }
        }

    
        print!("Type 'stop' to exit the program or press enter key to continue: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if input.trim().eq_ignore_ascii_case("stop") {
                println!("Ending pogram.");
                break;
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}