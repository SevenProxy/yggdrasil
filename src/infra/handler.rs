// 📦 Import modules
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::adapters::commands::{nick_command, join_command};
use  crate::error::ServerError;
use crate::{SharedState, SharedChannels};
use crate::adapters::presenters::{MessageClint, SafeTcpStream};

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
        Some(&"MODE") if parts.len() > 2 => {
          let target = parts[1].to_string();
          let flags = parts[2].to_string();
          if flags == "+i" {
              let message = format!(":{} MODE {} +i\r\n", "server", target);
              class_message_client.send_message_client(message).await?;
          } else {
              let message_waring = format!(":server 501 :Invalid mode\r\n");
              class_message_client.send_message_client(message_waring).await?;
          }
        }
        Some(&"USER") if parts.len() > 3 => {
          let username = parts[1].to_string();
          let hostname = parts[2].to_string(); // Host/client que ta se conectando.
          
          let message_welcome_client = format!(":server 001 {} {} :Bem-vindo ao servidor IRC\r\n", username, hostname);
          class_message_client.send_message_client(message_welcome_client).await?;
      
          let message_username_success = format!(":{} NICK {}\r\n", "server", username);
          class_message_client.send_message_client(message_username_success).await?;
        }
        Some(&"PING") => {
          if parts.len() > 1 {
            let token = parts[1];
            println!("{:?}", token);
            let message_ping = format!("PONG {}\r\n", token);
            class_message_client.send_message_client(message_ping).await?;
          }
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
        Some(&"PRIVMSG") if parts.len() > 2 => {
          let target = parts[1];
          let msg = parts[2..].join(" ");
  
          if target.starts_with("#") {
            let broadcast_message = format!(":{} PRIVMSG {} {}\r\n", nick, target, msg);
            class_message_client.broadcast(&target, &broadcast_message).await?;
          } else {
            let message_private = format!(":{} {} {}", nick, target, msg);
            class_message_client.send_message_client(message_private).await?;
          }
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
