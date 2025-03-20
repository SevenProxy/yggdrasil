// 📦 Import modules
use tokio::io::AsyncWriteExt;
use crate::error::ServerError;
use crate::SharedChannels;

use super::SafeTcpStream;

pub struct MessageClint {
  pub safe_socket: SafeTcpStream,
  pub channels: SharedChannels,
}

impl MessageClint {
  pub fn new(safe_socket: SafeTcpStream, channels: SharedChannels) -> Self {
    Self {
      safe_socket,
      channels,
    }
  }

  pub async fn send_message_client(&mut self, message: String) -> Result<(), ServerError> {
    match self.safe_socket.write(message.as_bytes()).await {
      Ok(_) => Ok(()),
      Err(e) => {
        let warning_message = format!("Erro ao enviar mensagem: {:?}", e);
        println!("{:?}", warning_message);
        Err(ServerError::Io(e))
      }
    }
  }

  pub async fn recive_client_message(&mut self, mut buffer: [u8; 1024]) -> Result<String, ServerError> {
    // Analisando os dados recebidos, caso seja "0", significa que a conexão foi fechada.
    //
    // let n: usize = 0 { return Ok(()) };
    let n = match self.safe_socket.read(&mut buffer).await {// Função read, ler dados do client, no caso, da conexão estabelecida (Socket)
      // ❌ Conexão fechada?
      Ok(0) => return Ok(String::new()),
      // ✅ Sucesso
      Ok(n) => n,
      Err(e) => {
        return Err(ServerError::Io(e));
      }
    };
    let message = String::from_utf8_lossy(&buffer[..n]).to_string();

    // Verificação simples para saber se a mensagem está vazia.
    //
    // if "".is_empty() = true;
    if message.is_empty() {
      return Ok(String::from(""));
    }

    return Ok(message);
  }

  pub async fn broadcast(&self, channel: &str, message: &str) -> Result<(), ServerError> {
    match self.channels.lock().await.get(channel).cloned() {
      Some(clients) => {
        for client in clients {
          let mut client_socket = client.lock().await;
          match client_socket.write_all(message.as_bytes()).await {
            Ok(_) => {
              let _ = client_socket.flush().await;
              println!("[Broadcast] Mensagem enviada para {}", channel);
            }
            Err(e) => println!("[Erro] Falha ao enviar mensagem: {}", e),
          }
        }
      }
      None => println!("Canal '{}' não encontrado.", channel),
    }
    Ok(())
  }

}

