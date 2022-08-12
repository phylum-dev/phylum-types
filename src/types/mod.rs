//! On the wire structs used for serializing / deserializing data for the CLI
//! client. Eventually this will include all such structs from the API side as
//! well to ease developing thirdparty clients

pub mod auth;
pub mod common;
pub mod group;
pub mod job;
pub mod package;
pub mod preferences;
pub mod project;
pub mod user_settings;
