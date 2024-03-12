use jsonrpsee::proc_macros::rpc;

pub const PROTOCOL_VERSION: &str = "0.0.1";

#[rpc(client)]
pub trait Rpc {
    /// Protocol version.
    #[method(name = "version")]
    fn version(&self) -> Result<String, ErrorObjectOwned>;

    /// Ping.
    #[method(name = "ping")]
    async fn ping(&self) -> Result<String, ErrorObjectOwned>;
}
