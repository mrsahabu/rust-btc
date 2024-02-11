use axum::response::Html;
use axum::routing::get;
use axum::Router;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::{bitcoin::Network, database::SqliteDatabase, SyncOptions, Wallet};
use dotenv::from_filename;
use std::env;
use std::net::SocketAddr;
// use tokio::net::unix::SocketAddr;

fn setup() -> String {
    from_filename(".env");
    let descriptor = env::var("WALLET_DESCRIPTOR");
    match descriptor {
        Ok(descriptor) => return descriptor,
        Err(_) => return "not found...!".to_owned(),
    };
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Sahab</h1>")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let descriptor = setup();
    // let db_path  = Path::new("btc.db")
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        SqliteDatabase::new("btc.db"),
    )?;
    //Axum Begins:
    let app = Router::new().route("/", get(handler));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    // wallet.sync(&blockchain, SyncOptions::default())?;
    // dbg!(&wallet);
    // let balance = wallet.get_balance()?;
    // dbg!(balance);
    // let address = wallet.get_address(AddressIndex::New);
    // dbg!(address);

    Ok(())
}
