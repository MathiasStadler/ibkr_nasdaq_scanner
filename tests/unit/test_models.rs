#[test]
fn test_option_type_display() {
    use ibkr_nasdaq_scanner::models::OptionType;
    
    let call = OptionType::Call;
    let put = OptionType::Put;
    
    assert_eq!(call.to_string(), "CALL");
    assert_eq!(put.to_string(), "PUT");
    
    assert_eq!(call.emoji(), "📈");
    assert_eq!(put.emoji(), "📉");
}

#[test]
fn test_option_chain_creation() {
    use ibkr_nasdaq_scanner::models::{OptionChain, OptionType};
    use chrono::{TimeZone, Utc};
    
    let expiration = Utc.with_ymd_and_hms(2024, 12, 20, 0, 0, 0).unwrap();
    
    let chain = OptionChain {
        symbol: "AAPL".to_string(),
        option_type: OptionType::Call,
        expiration,
        strike: 180.0,
        option_price: 2.50,
        stock_price: 185.0,
        profit_percent: 20.0,
        volume: 1000,
        implied_volatility: 0.25,
        timestamp: Utc::now(),
        contract_id: "123456".to_string(),
    };
    
    assert_eq!(chain.symbol, "AAPL");
    assert_eq!(chain.strike, 180.0);
    assert_eq!(chain.profit_percent, 20.0);
}
