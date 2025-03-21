use irc_server::{Arc, HashMap, Handler, Mutex, ServerError, TcpListener, Ipv4Addr, SocketAddr, IpAddr};

// 🧠 Start do serviço/protocolo. 6667 <- Para não criptografia e 6697 <- Para criptografia.
#[tokio::main]
async fn main() -> Result<(), ServerError> {
  let ip = Ipv4Addr::new(0, 0, 0, 0);
  let port = 6667;
  let socket_addr: SocketAddr = SocketAddr::new(IpAddr::V4(ip), port);


  let state = Arc::new(Mutex::new(HashMap::new()));
  let channels: Arc<_> = Arc::new(Mutex::new(HashMap::new()));
  // let listener: TcpListener = TcpListener::bind("localhost:6697").await?;
  let listener = TcpListener::bind(socket_addr).await?;

  println!("Servidor IRC rodando em {}", listener.local_addr()?);

  loop {
    // Esperando uma conexão, no caso um novo client.
    // let (client: TcpStream, _) = listener.accept.await?;
    let (socket, _) = listener.accept().await?;

    let socket = Arc::new(Mutex::new(socket));
    let state = Arc::clone(&state);
    let channels = Arc::clone(&channels);

    tokio::spawn(async move {
      let mut handle_client = Handler::new(socket, state, channels);
      if let Err(e) = handle_client.client().await {
        println!("Erro ao lidar com cliente: {}", e);
      }
      // if let Err(e) = handle_client(socket, state).await {
      //   Talvez eu faça para encerrar o loop, não sei ainda.
      //   println!("Erro ao lidar com cliente: {}", e);
      // }
    });
  }
}
