# mobc-arangors

Implementation of async connection pool for ```arangors``` using ```mobc```. Currently only ```reqwest``` is support as a client.

## Example

```rust
use mobc::Pool;
use std::time::Instant;
use mobc_arangors::ArangoDBConnectionManager;

#[tokio::main]
async fn main() {
    let manager = ArangoDBConnectionManager::new("http://arangoserver/", "root", "password", true);
    let pool = Pool::builder().max_open(20).build(manager);
    const MAX: usize = 100;

    let now = Instant::now();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<usize>(16);
    for i in 0..MAX {
        let pool = pool.clone();
        let mut tx_c = tx.clone();
        tokio::spawn(async move {

            let client = pool.get().await.unwrap();
            let db = client.db("_system").await.unwrap();

            let version = db.arango_version().await.unwrap().version;
            assert_eq!(version, "3.6.2");
            tx_c.send(i).await.unwrap();
        });
    }

    for _ in 0..MAX {
        rx.recv().await.unwrap();
    }

    println!("cost: {:?}", now.elapsed());
}
```