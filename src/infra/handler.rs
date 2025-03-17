use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::adapters::commands::nick_command;
use  crate::error::ServerError;
use crate::{SharedState, SharedChannels};
use crate::adapters::presenters::MessageClint;

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
    let mut buffer = [0; 1024];
    #[allow(unused_assignments)]
    let mut nick = String::new();

    loop {
      let mut socket = self.socket.lock().await;

      // Analisando os dados recebidos, caso seja "0", significa que a conexão foi fechada.
      //
      // let n: usize = 0 { return Ok(()) };
      let n = match socket.read(&mut buffer).await {// Função read, ler dados do client, no caso, da conexão estabelecida (Socket)
        // ❌ Conexão fechada?
        Ok(0) => return Ok(()),
        // ✅ Sucesso
        Ok(n) => n,
        Err(e) => {
          println!("Erro ao receber mensagem: {:?}", e);
          return Ok(());
        }
      };
      println!("{}", n);
      let message = String::from_utf8_lossy(&buffer[..n]);

      // Verificação simples para saber se a mensagem está vazia.
      //
      // if "".is_empty() = true;
      if message.is_empty() {
        return Ok(());
      }

      // Transformando a mensagem envaida pelo Client em vetor e passando-a para uma variavel.
      //
      // let parts = [1, 2, 3];
      let parts: Vec<&str> = message.split_whitespace().collect();

      println!("{:?}", parts);
      // if message.starts_with("NICK") {}
      match parts.get(0) {
        Some(&"PING") => {
          if parts.len() > 1 {
            let token = parts[1];
            println!("{:?}", token);
            socket.write_all(format!("PONG {}\r\n", token).as_bytes()).await?; // Função para enviar dados para o client (conexão estabelecida). Aceita apenas via bytes.
          }
        }
        Some(&"CAP") => {
          socket.write_all(b":server CAP * LS :\r\n").await?;
        }
        Some(&"NICK") => {
          nick = parts[1].to_string();
          let message_client = MessageClint::new(socket);
          let _ = nick_command(message_client, parts).await;
        }
        
        Some(&"JOIN") if parts.len() > 1 => {
          let channel = parts[1].to_string();
          let mut channels = self.channels.lock().await;
          
          let join_message = format!(":{} JOIN {}\r\n", nick, channel);
          socket.write_all(join_message.as_bytes()).await?;
          
          //channels.entry(channel.clone()).or_insert_with(Vec::new).push(self.socket.());
          //
          let entry = channels.entry(channel.to_string()).or_insert_with(Vec::new);
          entry.push(Arc::clone(&self.socket));
        }
        Some(&"PRIVMSG") if parts.len() > 2 => {
          let target = parts[1];
          let msg = parts[2..].join(" ");

          if target.starts_with("#") {
            let broadcast_message = format!(":{} PRIVMSG {} :{}\r\n", nick, target, msg);
            self.broadcast(&target, &broadcast_message).await?;
          } else {
            let message_private = format!(":{} {} {}", nick, target, msg);
            socket.write_all(message_private.as_bytes()).await?;
          }
        }
        _ => {
          socket.write_all(b":server 421 :Comando desconhecido\r\n").await?;
        }
      }
    }
  }

  pub async fn broadcast(&self, channel: &str, message: &str) -> Result<(), ServerError> {
    let clients = {
      let channels = self.channels.lock().await;
      channels.get(channel).cloned()
    };

    if let Some(clients) = clients {
      if clients.is_empty() {
        println!("O canal '{}' não tem clientes.", channel);
      } else {
        for client in clients {
          let mut client_socket = client.lock().await;
          if let Err(_) = client_socket.write_all(message.as_bytes()).await {
            println!("Erro ao enviar mensagem!");
          } else {
            println!("Mandou mensagem!");
          }
        }
      }
    } else {
      println!("Canal '{}' não encontrado.", channel);
    }
    Ok(())
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
