use crate::ProtoVersion;
use bedrock_macros::{ProtoCodec, packet};

#[packet(id = 85)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct TransferPlayerPacket<V: ProtoVersion> {
    pub server_address: String,
    #[endianness(le)]
    pub server_port: u16,
    /// Whether the client should reload the world on transfer.
    ///
    /// Restored: this field is present in the wire format (gophertunnel's
    /// `Transfer.Marshal` writes `Address`, `Port`, `ReloadWorld`, then the
    /// optional gathering info) but was absent here, making every
    /// `TransferPlayer` one byte short. A real client reads the optional's
    /// presence flag as this field, runs out of packet, and silently discards
    /// the transfer — it never dials the destination at all.
    pub reload_world: bool,
    pub gatherings_config: Option<V::GatheringsConfig>,
}
