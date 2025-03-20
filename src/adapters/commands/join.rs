use std::sync::Arc;

// 📦 Import modules
use crate::adapters::presenters::MessageClint;

// Comando JOIN
//
// Quando o client entra em algum canal, o client IRC envia um comando para a confirmação, em seguida o server IRC responde a esse comando..
pub async fn join_command(mut message_client: MessageClint, parts: Vec<&str>, nick: String) -> Result<(), ()> {
  match parts.len() > 1 {
    true => {
      let channel = parts[1].to_string();
      let join_message = format!(":{} JOIN {}\r\n", nick, channel);
      let _ = message_client.send_message_client(join_message).await;

      let mut channels = message_client.channels.lock().await;
      //channels.entry(channel.clone()).or_insert_with(Vec::new).push(self.socket.());
      //
      let entry = channels.entry(channel.to_string()).or_insert_with(Vec::new);
      entry.push(Arc::clone(&message_client.safe_socket.socket));
      //drop(socket);
      //let join_message = format!(":{} JOIN {}\r\n", nick, channel);
      //self.broadcast(&channel, &join_message).await?;
      Ok(())
    }
    false => {
      let undefined_message = format!(":server 421 :Comando não encontrado.\r\n");
      let _ = message_client.send_message_client(undefined_message).await;
      Err(())
    }
  }
}
