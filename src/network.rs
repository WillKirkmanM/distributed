use crate::command::{parse, Command};
use crate::store::KeyValueStore;
use crate::transaction::Transaction;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
    store: KeyValueStore,
}

impl Server {
    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).await.unwrap();
        let store = KeyValueStore::new();
        Server { listener, store }
    }

    pub async fn run(self) {
        println!("Distributed Server listening on {}", self.listener.local_addr().unwrap());
        loop {
            let (socket, _) = self.listener.accept().await.unwrap();
            let store = self.store.clone();
            tokio::spawn(async move {
                handle_connection(socket, store).await;
            });
        }
    }
}

async fn handle_connection(mut socket: TcpStream, store: KeyValueStore) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Each connection gets its own transaction state
    let mut tx = Transaction::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                let response = match parse(&line) {
                    Ok(cmd) => process_command(cmd, &mut tx, &store),
                    Err(e) => format!("ERR: {}\n", e),
                };
                if writer.write_all(response.as_bytes()).await.is_err() {
                    break;
                }
                line.clear();
            }
            Err(_) => break,
        }
    }
}

fn process_command(cmd: Command, tx: &mut Transaction, store: &KeyValueStore) -> String {
    if !tx.is_active() && !matches!(cmd, Command::Begin | Command::Get(_)) {
        return "ERR: Must be inside a transaction to SET, DEL, COMMIT, or ROLLBACK\n".to_string();
    }

    match cmd {
        Command::Begin => match tx.begin() {
            Ok(_) => "OK\n".to_string(),
            Err(e) => format!("ERR: {}\n", e),
        },
        Command::Commit => match tx.commit(store) {
            Ok(_) => "OK\n".to_string(),
            Err(e) => format!("ERR: {}\n", e),
        },
        Command::Rollback => {
            tx.rollback(store);
            "OK\n".to_string()
        }
        Command::Set(_, _) | Command::Del(_) => match tx.stage_write(cmd) {
            Ok(_) => "QUEUED\n".to_string(),
            Err(e) => format!("ERR: {}\n", e),
        },
        Command::Get(key) => match tx.get_value(&key, store) {
            Some(val) => format!("\"{}\"\n", val),
            None => "(nil)\n".to_string(),
        },
    }
}
