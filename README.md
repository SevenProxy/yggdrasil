
# Contexto

**O `servidor IRC` é software que utiliza o protocolo `TCP`, para receber e enviar dados para a conexão estabelecida.**
O Projeto utiliza o modulo/lib [tokio](https://tokio.rs/) para iniciar essa conexão.
No arquivo `main.rs`, temos a conexão sendo iniciada:
`/main.rs`
```rs
pub use tokio::net::{TcpListener, TcpStream}; // Essa chamada do modulos já ocorre dentro de `lib.rs`, isso é apenas uma demostração.
use irc_server::{Handler, Mutex, ServerError, TcpListener};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
  let listener: TcpStream = TcpListener::bind("127.0.0.1:6667").await?;

  // Precisamos deixa-la em loop, para receber multiplas conexões.
  loop {
    // Aqui usamos o metodo accept para receber uma conexão. Veja que socket passa a ser a conexão vinda do client.
    let (socket: TcpStream, _) = listener.accept().await?;

    // Em seguida, para evitar erros de multiplas conexões, ou sobrescrever conexões existentes, usamos o Mutext da lib/modulo Tokio.
    //
    // Com isso, garantimos que a conexão estabelecida não seja alterada ou sobescrita, deixando-as com um fluxo de controle melhor, pois assim teremos um controle de conexão único para cada client conectado.
    let socket = Arc::new(Mutex::new(socket));
  }
}
```
