// 📦 Export module
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::TcpStream, sync::Mutex};

pub struct SafeTcpStream {
  pub socket: Arc<Mutex<TcpStream>>,
}

impl SafeTcpStream {
  pub async fn write(&self, data: &[u8]) -> tokio::io::Result<()> {
    drop(self.socket.clone());
    let mut socket = self.socket.lock().await;
    socket.write_all(data).await
  }
  
  pub async fn read(&self, buffer: &mut [u8]) -> tokio::io::Result<usize> {
    let mut socket = self.socket.lock().await;
    socket.read(buffer).await
  }
}
  