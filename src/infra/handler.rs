// 📦 Import modules
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use std::sync::Arc;

use  crate::error::ServerError;
use crate::{SharedState, SharedChannels};
use crate::adapters::presenters::{MessageClint, SafeTcpStream};
use crate::adapters::commands::{nick_command, join_command, privmsg_command, ping_command, mode_command};

pub struct Handler {
  pub socket: Arc<Mutex<TcpStream>>,
  #[allow(dead_code)]
  pub state: SharedState,
  pub channels: SharedChannels,
}

impl Handler {

  pub fn new(socket: Arc<Mutex<TcpStream>>, state: SharedState, channels: SharedChannels) -> Self {
    Self {
      socket,
      state,
      channels
    }
  }

  pub async fn client(&mut self) -> Result<(), ServerError> {
    let buffer = [0; 1024];
    #[allow(unused_assignments)]
    let mut nick = String::new();

    loop {
      //let socket = self.socket.lock().await;
      let safe_tcp_stream = SafeTcpStream {
        socket:  self.socket.clone(),
      };
      let mut class_message_client = MessageClint::new(safe_tcp_stream, self.channels.clone());
      let result_message_recive = class_message_client.recive_client_message(buffer).await?;
      // Transformando a mensagem envaida pelo Client em vetor e passando-a para uma variavel.
      //
      // let parts = [1, 2, 3];
      let parts: Vec<&str> = result_message_recive.split_whitespace().collect();

      println!("{:?}", parts);
      // if message.starts_with("NICK") {}
      match parts.get(0) {
        Some(&"MODE") => {
          let _ = mode_command(class_message_client, parts).await;
        }
        Some(&"PING") => {
          let _ = ping_command(class_message_client, parts).await;
        }
        Some(&"CAP") => {
          let message_cap = format!(":server CAP * LS :<capability>\r\n");
          class_message_client.send_message_client(message_cap).await?;
        }
        Some(&"NICK") => {
          nick = parts[1].to_string();
          let _ = nick_command(class_message_client, parts).await;
        }
        Some(&"JOIN") => {
          let _ = join_command(class_message_client, parts, nick.clone()).await;
        }
        Some(&"PRIVMSG") => {
          let _ = privmsg_command(class_message_client, parts, nick.clone()).await;
        }
        _ => {
          let message_command_undefined = format!(":server 421 :Comando desconhecido\r\n");
          class_message_client.send_message_client(message_command_undefined).await?;
        }
      }
    }
  }

}


// pub struct Server {
//     clients: Arc<Mutex<HashMap<String, Client>>>,
// }
//
// impl Server {
//     pub fn new() -> Self {
//         Server {
//             clients: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }
//
//     pub async fn add_client(&self, nickname: String, client: Client) {
//         let mut clients = self.clients.lock().await;
//         clients.insert(nickname, client);
//     }
//
//     pub async fn send_message(&self, target: &str, message: &str) {
//         let clients = self.clients.lock().await;
//         if let Some(client) = clients.get(target) {
//             let mut stream = client.stream.lock().await;
//             stream.write_all(message.as_bytes()).await.unwrap();
//         }
//     }
// }
