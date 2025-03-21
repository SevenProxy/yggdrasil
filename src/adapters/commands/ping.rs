// 📦 Import modules
use crate::adapters::presenters::MessageClint;

// Comando JOIN
//
// Quando o client entra em algum canal, o client IRC envia um comando para a confirmação, em seguida o server IRC responde a esse comando..
pub async fn ping_command(mut message_client: MessageClint, parts: Vec<&str>) -> Result<(), ()> {
  match parts.len() > 1 {
    true => {
      let token = parts[1];
      let message_ping = format!("PONG {}\r\n", token);
      let _ = message_client.send_message_client(message_ping).await;
      Ok(())
    }
    false => {
      let undefined_message = format!(":server 421 :Comando não encontrado.\r\n");
      let _ = message_client.send_message_client(undefined_message).await;
      Err(())
    }
  }
}
