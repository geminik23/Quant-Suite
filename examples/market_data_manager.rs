// use std::collections::HashMap;
// use std::env;
use std::error::Error;
//
// use cfix::types::CTraderLogin;
// use cfix::utilities::market_data::MarketDataManager;
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //     dotenv::dotenv().ok();
    //     env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    //
    //     let host = env::var("CTRADER_FIX_HOST").unwrap();
    //     let username = env::var("CTRADER_FIX_USERNAME").unwrap();
    //     let password = env::var("CTRADER_FIX_PASSWORD").unwrap();
    //     let sendercompid = env::var("CTRADER_FIX_SENDERCOMPID").unwrap();
    //
    //     let login = CTraderLogin {
    //         username,
    //         password,
    //         server: host,
    //         sendercompid,
    //         ssl: false,
    //         heartbeat_interval: None,
    //     };
    //
    //     //
    //     let mut mdm = MarketDataManager::new();
    //     let symbols = mdm.get_onetime_symbol_list(login.clone()).await?;
    //     // log::info!("{:?}", symbols);
    //
    //     let mut mapping: HashMap<u32, String> = HashMap::new();
    //     mapping.insert(1, "eurusd".into());
    //
    //     let conn_id = mdm.connect(login, mapping).await?;
    //
    //     mdm.subscribe_spot(conn_id, vec!["eurusd"]).await?;
    //
    Ok(())
}
