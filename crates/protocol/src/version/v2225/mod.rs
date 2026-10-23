//! r/26_u6
//!
//! A handshake-scoped module for 1.26.60 retail, taken from upstream bedrock-crustaceans
//! (`crates/protocol/src/version/v2225`, which sits on a restructured `main` this fork has
//! not rebased onto). On the login-to-`StartGame` path upstream's 2225 differs from 2193 in
//! exactly two places: `LevelSettings` gained `editor_level_migration_version` — already
//! carried by this fork's v2216 and inherited from it unchanged — and
//! `ConnectionFailReason` gained `SubClientLoginNotBound = 150`, copied here verbatim.
//! Everything else is inherited from 2216/2193, which is **wrong for the rest of the
//! protocol** (2225 also changed `PlayerList`, `LevelChunk`, `SerializedSkin`, `Animate`,
//! the inventory transactions and more, and added eight packets). Replace this module with
//! upstream's once the fork is rebuilt on upstream `main`.

pub mod enums;
