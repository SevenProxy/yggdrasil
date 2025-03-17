use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::MutexGuard;

use crate::error::ServerError;

pub struct MessageClint<'a> {
  socket: MutexGuard<'a, TcpStream>,
}

impl<'a> MessageClint<'a> {
  pub fn new(socket: MutexGuard<'a, TcpStream> ) -> Self {
    Self {
      socket,
    }
  }
  pub async fn send_message_client(&mut self, message: String) -> Result<(), ServerError> {
    match self.socket.write_all(message.as_bytes()).await {
      Ok(_) => Ok(()),
      Err(e) => {
        let warning_message = format!("Erro ao enviar mensagem: {:?}", e);
        println!("{:?}", warning_message);
        Err(ServerError::Io(e))
      }
    }
  }

  // Fazer o metodo para buscar os dados vindo do client (tirar essa responsa do handler)
}

