pub fn init_test_logging() {
    _ = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .try_init();
}