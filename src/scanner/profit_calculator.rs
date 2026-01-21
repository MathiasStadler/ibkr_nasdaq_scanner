use crate::models::OptionType;

pub fn calculate_profit_percent(
    strike: f64,
    option_price: f64,
    stock_price: f64,
    option_type: &OptionType,
) -> f64 {
    match option_type {
        OptionType::Call => calculate_call_profit(strike, option_price, stock_price),
        OptionType::Put => calculate_put_profit(strike, option_price, stock_price),
    }
}

fn calculate_call_profit(strike: f64, option_price: f64, stock_price: f64) -> f64 {
    if strike <= 0.0 || option_price <= 0.0 {
        return 0.0;
    }
    
    // Simplified profit calculation for calls
    // (Stock price - Strike price - Option price) / Option price * 100
    if stock_price > strike {
        ((stock_price - strike - option_price) / option_price * 100.0).max(0.0)
    } else {
        0.0
    }
}

fn calculate_put_profit(strike: f64, option_price: f64, stock_price: f64) -> f64 {
    if strike <= 0.0 || option_price <= 0.0 {
        return 0.0;
    }
    
    // Simplified profit calculation for puts
    // (Strike price - Stock price - Option price) / Option price * 100
    if strike > stock_price {
        ((strike - stock_price - option_price) / option_price * 100.0).max(0.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::OptionType;

    #[test]
    fn test_call_profit_calculation() {
        // In the money call
        let profit = calculate_call_profit(100.0, 5.0, 110.0);
        assert!((profit - 100.0).abs() < 0.01); // (110-100-5)/5*100 = 100%
        
        // Out of the money call
        let profit = calculate_call_profit(100.0, 2.0, 95.0);
        assert_eq!(profit, 0.0);
    }
    
    #[test]
    fn test_put_profit_calculation() {
        // In the money put
        let profit = calculate_put_profit(100.0, 5.0, 90.0);
        assert!((profit - 100.0).abs() < 0.01); // (100-90-5)/5*100 = 100%
        
        // Out of the money put
        let profit = calculate_put_profit(100.0, 2.0, 105.0);
        assert_eq!(profit, 0.0);
    }
    
    #[test]
    fn test_profit_calculator() {
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
}
