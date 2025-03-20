// 📦 Import modules
use crate::adapters::presenters::MessageClint;

// Comando NICK
//
// O comando NICK é para assim que o cliente se conectar, setar o NICK ou HOSTNAME no cliente IRC.
pub async fn nick_command(mut message_client: MessageClint, parts: Vec<&str>) -> Result<(), ()> {
  match parts.len() > 1 {
    true => {
      let nick = parts[1].to_string();
      let message_welcome_client = format!(":server 001 {} {} :Bem-vindo ao servidor IRC\r\n", nick, nick);
      let _ = message_client.send_message_client(message_welcome_client).await;
      // Resposta ao client IRC com o seu NICK.
      let _ = message_client.send_message_client(format!(":{} NICK {}\r\n", "server", nick)).await;
      Ok(())
    }
    false => {
      let undefined_message = format!(":server 421 :Comando não encontrado.\r\n");
      let _ = message_client.send_message_client(undefined_message).await;
      Err(())
    }
  }
}
