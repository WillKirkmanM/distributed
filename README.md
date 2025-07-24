<p align="center">
  <img src="https://avatars.githubusercontent.com/u/138057124?s=200&v=4" width="150" />
</p>
<h1 align="center">Distributed</h1>

<p align="center">Distributed Systems Framework with Key-Value Store with RAFT Distributed Consensus Algorithm, Thread-Safe Data Structures, Transactions & TCP Networking</p>

* **Distributed Consensus:** Distributed uses the Raft consensus algorithm to ensure that all nodes in the cluster agree on the state of the data.
* **Thread-Safe Data Structures:** The core data store is implemented using thread-safe data structures to allow for concurrent access from multiple clients.
* **Transactions:** Distributed supports atomic transactions, allowing multiple operations to be grouped together and either all succeed or all fail.
* **Networking:** The system uses a custom networking protocol built on top of TCP to allow nodes to communicate with each other.

## Getting Started

To get started with Distributed, you'll need to have Rust installed on your system. You can then clone the repository and build the project:

```bash
git clone [https://github.com/WillKirkmanM/distributed](https://github.com/WillKirkmanM/distributed)
cd distributed
cargo build --release
```

Once the project is built, you can start a new node in the cluster:

```bash
./target/release/distributed --id 1 --port 8080
```

This will start a new node with the ID `1` on port `8080`. To add more nodes to the cluster, you can start them on different ports and with different IDs.

## Interacting with the Cluster

You can interact with the cluster using a simple command-line interface (CLI). To start the CLI, run the following command:

```rust
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:8080";
    println!("Connecting to Distributed server at {}", server_addr);

    let stream = match TcpStream::connect(server_addr).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return;
        }
    };
    println!("Connected. Type your commands.");

    let (reader, mut writer) = io::split(stream);
    let mut reader = BufReader::new(reader);
    let mut stdin_reader = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut response = String::new();

    loop {
        print!("> ");
        io::stdout().flush().await.unwrap();

        line.clear();
        match stdin_reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                writer.write_all(line.as_bytes()).await.unwrap();

                response.clear();
                reader.read_line(&mut response).await.unwrap();
                print!("{}", response);
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
}
```

The CLI provides commands for getting and setting values in the key-value store:

```
> SET mykey myvalue
OK
> GET mykey
"myvalue"
```

You can also use transactions to group multiple operations together:

```
> BEGIN
OK
> SET key1 value1
OK
> SET key2 value2
OK
> COMMIT
OK
```

## How it Works

Distributed is built on top of the Raft consensus algorithm, which is a protocol for managing a replicated log. In Distributed, each node in the cluster maintains a copy of the key-value store, and all changes to the store are written to a log.

The Raft algorithm ensures that the logs on all nodes are consistent, which means that all nodes will have the same view of the data. When a client sends a request to a node, the node forwards the request to the leader of the cluster. The leader then appends the request to its log and replicates it to the other nodes. Once a majority of nodes have acknowledged the request, the leader applies the change to its key-value store and sends a response to the client.

This process ensures that all changes to the key-value store are atomic and consistent, even in the face of node failures.
