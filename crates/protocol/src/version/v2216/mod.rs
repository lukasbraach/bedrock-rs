//! r/26_u6
//!
//! A handshake-scoped module, written by hand from Mojang/bedrock-protocol-docs
//! `v1.26.60-preview.28` rather than generated from a complete diff. It carries the one
//! wire change the login-to-`StartGame` path meets (`LevelSettings` gained
//! `Editor Level Migration Version`) and inherits everything else from 2193, which is
//! **wrong for the rest of the protocol**: 2216 also changed `PlayerList`, `LevelChunk`,
//! `SerializedSkin`, `Animate`, the inventory transactions and more, and added five
//! packets. Replace this module with upstream's once bedrock-crustaceans ships 2216.

pub mod types;
