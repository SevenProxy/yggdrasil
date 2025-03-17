// 📦 Import modules
mod infra;
mod adapters;
mod error;

pub use tokio::net::{TcpListener, TcpStream};  // 📦 Usando a lib tokio para abrir uma conexão aberta usando protocolo TCP.
pub use tokio::sync::Mutex;
pub use std::sync::Arc;
pub use std::collections::HashMap;
pub use infra::handler::Handler;
pub use error::ServerError;

// 📦 Export types
pub type SharedState = Arc<Mutex<HashMap<String, TcpStream>>>;
pub type SharedChannels = Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>;
