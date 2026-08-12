#[cfg(feature = "nethernet")]
use crate::error::NetherNetError;
use crate::error::{RakNetError, TransportLayerError};
use crate::info::RAKNET_GAMEPACKET_ID;
use byteorder::{ReadBytesExt, WriteBytesExt};
use raknet_tokio::prelude::*;
use std::io::{Cursor, Write};

pub enum TransportLayerConnection {
    RakNet(RakSession),
    /// A NetherNet peer connection: Bedrock over a WebRTC data channel.
    ///
    /// This is the transport a Nintendo Switch and a Bedrock client on a phone use
    /// for LAN play; there is no RakNet path to a Switch at all.
    ///
    /// Feature-gated because it pulls in `webrtc`, which is around 1.6 MB of
    /// dependencies and raises the Android minimum to API 24 (`webrtc-util`
    /// enumerates interfaces through `nix::ifaddrs`, and Bionic only gained
    /// `getifaddrs` there). A RakNet-only consumer should not pay for either.
    #[cfg(feature = "nethernet")]
    NetherNet(nethernet::NethernetStream),
    // TODO: Quic(s2n_quic::stream::BidirectionalStream),
    // TODO: Tcp(net::TcpStream),
}

impl TransportLayerConnection {
    pub async fn send(&mut self, stream: &[u8]) -> Result<(), TransportLayerError> {
        match self {
            Self::RakNet(conn) => {
                // 1 = RAKNET_GAMEPACKET_ID size
                let mut buf = Vec::with_capacity(stream.len() + 1);

                // TODO Find out a way to avoid copying of the entire buffer
                buf.write_u8(RAKNET_GAMEPACKET_ID)?;
                buf.write_all(stream)?;

                // TODO Find out if immediate: true should be used
                conn.send(buf, RakReliability::ReliableOrdered, RakPriority::Immediate)
                    .await
                    .map_err(RakNetError::from)?;
            }
            // No RAKNET_GAMEPACKET_ID. NetherNet prepends a remaining-segment
            // counter and nothing else — go-nethernet's `Conn.BatchHeader` returns
            // nil — and the segmenting happens below this layer, in the stream.
            #[cfg(feature = "nethernet")]
            Self::NetherNet(conn) => {
                conn.send(bytes::Bytes::copy_from_slice(stream))
                    .await
                    .map_err(NetherNetError::from)?;
            }
        }

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<u8>, TransportLayerError> {
        let stream = match self {
            Self::RakNet(conn) => {
                let stream: Vec<u8> = conn.recv().await.map_err(RakNetError::from)?;

                let mut stream = Cursor::new(stream);

                // Read the RakNet Packet ID
                let raknet_packet_id = stream.read_u8()?;

                if raknet_packet_id != RAKNET_GAMEPACKET_ID {
                    return Err(TransportLayerError::RakNetError(
                        RakNetError::InvalidRakNetHeader(raknet_packet_id),
                    ));
                };

                let mut stream = stream.into_inner();
                stream.drain(..1);

                stream
            }
            // Deliberately no header read and no drain: the first byte belongs to the
            // batch. Measured against a Nintendo Switch on Bedrock 26.41, whose first
            // data-channel batch began `06 c1 01 …` — a varint length, not 0xfe.
            #[cfg(feature = "nethernet")]
            Self::NetherNet(conn) => conn
                .recv()
                .await
                .map_err(NetherNetError::from)?
                .ok_or(TransportLayerError::NetherNetClosed)?
                .to_vec(),
        };

        Ok(stream)
    }

    pub async fn close(&self) {
        match self {
            Self::RakNet(conn) => {
                let _ = conn.close().await;
            }
            #[cfg(feature = "nethernet")]
            Self::NetherNet(conn) => {
                let _ = conn.close().await;
            }
        }
    }

    pub async fn is_closed(&self) -> bool {
        match self {
            Self::RakNet(conn) => conn.is_closed().await,
            #[cfg(feature = "nethernet")]
            Self::NetherNet(conn) => conn.session().is_closed().await,
        }
    }
}
