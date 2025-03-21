// 📦 Import modules
use crate::adapters::presenters::MessageClint;

// Comando JOIN
//
// Quando o client entra em algum canal, o client IRC envia um comando para a confirmação, em seguida o server IRC responde a esse comando..
pub async fn privmsg_command(mut message_client: MessageClint, parts: Vec<&str>, nick: String) -> Result<(), ()> {
  match parts.len() > 2 {
    true => {
      let target = parts[1];
      let msg = parts[2..].join(" ");
    
      if target.starts_with("#") {
        let broadcast_message = format!(":{} PRIVMSG {} {}\r\n", nick, target, msg);
        let _ = message_client.broadcast(&target, &broadcast_message).await;
      } else {
        let message_private = format!(":{} {} {}", nick, target, msg);
        let _ = message_client.send_message_client(message_private).await;
      }
      Ok(())
    }
    false => {
      let undefined_message = format!(":server 421 :Comando não encontrado.\r\n");
      let _ = message_client.send_message_client(undefined_message).await;
      Err(())
    }
  }
}
