#[test]
fn test_profit_calculations() {
    use ibkr_nasdaq_scanner::scanner::profit_calculator::*;
    use ibkr_nasdaq_scanner::models::OptionType;
    
    // Test call options
    assert!(calculate_call_profit(100.0, 5.0, 110.0) > 0.0);
    assert_eq!(calculate_call_profit(100.0, 5.0, 100.0), 0.0);
    
    // Test put options
    assert!(calculate_put_profit(100.0, 5.0, 90.0) > 0.0);
    assert_eq!(calculate_put_profit(100.0, 5.0, 100.0), 0.0);
    
    // Test wrapper function
    let call_profit = calculate_profit_percent(
        100.0,
        5.0,
        110.0,
        &OptionType::Call,
    );
    assert!(call_profit > 0.0);
    
    let put_profit = calculate_profit_percent(
        100.0,
        5.0,
        90.0,
        &OptionType::Put,
    );
    assert!(put_profit > 0.0);
}
