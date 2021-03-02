// extern crate shared2;

use shared;

#[tokio::main]
async fn main() {
    shared::sms_sender::send_confirm_sms("09015132328", 5).await;

    // shared2::rpc::RpcClient
}
