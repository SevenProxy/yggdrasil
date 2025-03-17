// 📦 Import modules
use crate::adapters::presenters::MessageClint;

pub async fn nick_command<'a>(mut message_client: MessageClint<'a>, parts: Vec<&str>) -> Result<(), ()> {
  match parts.len() > 1 {
    true => {
      let nick = parts[1].to_string();
      let message_welcome_client = format!(":server 001 {} :Bem-vindo ao servidor IRC\r\n", nick);
      let _ = message_client.send_message_client(message_welcome_client).await;
      Ok(())
    }
    false => {
      let undefined_message = format!(":server 421 :Comando Comando não encontrado.\r\n");
      let _ = message_client.send_message_client(undefined_message).await;
      Err(())
    }
  }
}
