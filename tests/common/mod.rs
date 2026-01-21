// Common test utilities

pub fn setup_test_env() {
    // Set up test environment variables
    std::env::set_var("IBKR_GATEWAY_HOST", "localhost");
    std::env::set_var("IBKR_GATEWAY_PORT", "5000");
    std::env::set_var("MIN_PROFIT_PERCENT", "2.0");
}
