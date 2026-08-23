/* Cargo.toml
[dependencies]
tokio ={version ="1.39.0", features=['full']}
tokio-util ='0.7.12'
tracing="0.1"
tracing-subscriber="0.3"

[dev-dependencies]
tokio = { version = "1.39.0", features = ["full"] }
*/

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    signal,
    sync::broadcast,
};
use tokio_util::sync::CancellationToken;
#[tokio::main]

async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    let token = CancellationToken::new();
    let cancel_token = token.clone();

    tokio::spawn({
        async move {
            match signal::ctrl_c().await {
                Ok(()) => {
                    tracing::warn!("shutdown tasks");
                    cancel_token.cancel();
                }
                Err(_err) => {}
            }
        }
    });
    loop {
        let token = token.clone();

        let tx = tx.clone();

        let mut rx = tx.subscribe();

        let (mut socket, address) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            tracing::info!("spawning new tasks");
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();

            let mut reader = BufReader::new(stream_reader);
            loop {
                tokio::select! {
                    result =reader.read_line(& mut message) =>{
                        tracing::info!("Recieved message from client: {}",&message);
                        if result.unwrap() ==0{
                            break;
                        }
                        tracing::info!("Transmitted message over channel:{}",&message);
                        tx.send((message.clone() , address)).unwrap();
                        message.clear();
                    }
                    result=rx.recv()=>{
                        let (recieved_message , sender_address)=result.unwrap();

                        if address!=sender_address{
                        tracing::info!("Transmitted message over channel:{}",&recieved_message);

                            stream_writer.write_all(recieved_message.as_bytes()).await.unwrap();
                        }
                    }
                    _=token.cancelled()=>{
                       tracing::info!("task clean up");
                        return;
                    }
                }
                let recieved_message = rx.recv().await.unwrap();
                stream_writer.write_all(message.as_bytes()).await.unwrap();
            }
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{
        net::TcpStream,
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        time::{sleep, Duration}
    };

    #[tokio::test]
    async fn test_full_tcp_broadcast() {
        // broadcast channel and token
        let (tx, _) = tokio::sync::broadcast::channel(10);
        let token = tokio_util::sync::CancellationToken::new();

        // spawn listener TCP random port
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let listener_token = token.clone();
        let listener_tx = tx.clone();

        tokio::spawn(async move {
            loop {
                let token = listener_token.clone();
                let tx = listener_tx.clone();
                let mut rx = tx.subscribe();
                let (socket, address) = listener.accept().await.unwrap();

                tokio::spawn(async move {
                    let (stream_reader, mut stream_writer) = socket.into_split();
                    let mut reader = BufReader::new(stream_reader);
                    let mut message = String::new();

                    loop {
                        tokio::select! {
                            result = reader.read_line(&mut message) => {
                                if result.unwrap() == 0 {
                                    break;
                                }
                                tx.send((message.clone(), address)).unwrap();
                                message.clear();
                            }
                            result = rx.recv() => {
                                let (rec_msg, sender_addr) = result.unwrap();
                                if address != sender_addr {
                                    stream_writer.write_all(rec_msg.as_bytes()).await.unwrap();
                                }
                            }
                            _ = token.cancelled() => {
                                return;
                            }
                        }
                    }
                });
            }
        });

        // Client 1
        let client1 = TcpStream::connect(addr).await.unwrap();
        let (reader1, mut writer1) = client1.into_split();
        let mut reader1 = BufReader::new(reader1);

        // Client 2
        let client2 = TcpStream::connect(addr).await.unwrap();
        let (reader2, mut writer2) = client2.into_split();
        let mut reader2 = BufReader::new(reader2);

        // Client 1 send message
        writer1.write_all(b"hello from client1\n").await.unwrap();

        // Client 2 recieves message
        let mut buf2 = String::new();
        reader2.read_line(&mut buf2).await.unwrap();
        assert_eq!(buf2, "hello from client1\n");

        // Client 2 sends message
        writer2.write_all(b"hello from client2\n").await.unwrap();

        // Client 1 recieves message
        let mut buf1 = String::new();
        reader1.read_line(&mut buf1).await.unwrap();
        assert_eq!(buf1, "hello from client2\n");

        //cancellation test
        token.cancel();
        sleep(Duration::from_millis(100)).await;
    }
}
