use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub fn encode_symbol(symbol: &str) -> String {
    utf8_percent_encode(symbol, NON_ALPHANUMERIC).to_string()
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn format_percent(value: f64) -> String {
    format!("{:.2}%", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_symbol() {
        assert_eq!(encode_symbol("AAPL"), "AAPL");
        assert_eq!(encode_symbol("BRK-B"), "BRK-B");
    }

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(123.456), "$123.46");
        assert_eq!(format_currency(0.0), "$0.00");
    }

    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(12.3456), "12.35%");
        assert_eq!(format_percent(0.0), "0.00%");
    }
}
