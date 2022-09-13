use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::yield_now;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // tokio::spawn(async {
    //     {
    //         let rc = Rc::new("Hello");
    //         println!("{}",rc);
    //     }
    //     // let rr = Rc::new(vec![12,3]);
    //     yield_now().await;
    // });
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        println!("Accepted");

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

// async fn process(socket: TcpStream) {
//     let mut connection = Connection::new(socket);

//     if let Some(frame) = connection.read_frame().await.unwrap() {
//         println!("GOT: {:?}", frame);

//         let response = Frame::Error("unimplemented".to_string());
//         connection.write_frame(&response).await.unwrap();
//     }
// }


async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // `mini-redis` が提供するコネクションによって、ソケットから来るフレームをパースする
    let mut connection = Connection::new(socket);

    // コネクションからコマンドを受け取るため `read_frame` を使う
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` はデータが Bytes` 型であることを期待する
                    // この型についてはのちほど解説する
                    // とりあえずここでは `.into()` を使って `&Vec<u8>` から `Bytes` に変換する
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // クライアントへのレスポンスを書き込む
        connection.write_frame(&response).await.unwrap();
    }
}