// 📦 Import modules
use crate::adapters::presenters::MessageClint;

// Comando JOIN
//
// Quando o client entra em algum canal, o client IRC envia um comando para a confirmação, em seguida o server IRC responde a esse comando..
pub async fn mode_command(mut message_client: MessageClint, parts: Vec<&str>) -> Result<(), ()> {
  match parts.len() > 1 {
    true => {
      let target = parts[1].to_string();
      let flags = parts[2].to_string();
      if flags == "+i" {
        let message = format!(":{} MODE {} +i\r\n", "server", target);
        let _ = message_client.send_message_client(message).await;
      } else {
        let message_waring = format!(":server 501 :Invalid mode\r\n");
        let _ = message_client.send_message_client(message_waring).await;
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
