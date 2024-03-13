pub fn init() {
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("none,universal_live_share_client_dbg=trace"),
    );
}
