mod kr;
use utils::env::envload;

use crate::kr::timer::kr_timer;

#[tokio::main]
async fn main() {
    envload();
    kr_timer().await
}
