use async_trait::async_trait;
use my_interface::SayHelloService;
use std::thread::sleep;
use std::time::Duration;
use tokio::{self, runtime::Handle};

#[no_mangle]
pub fn new_service(handle: Handle) -> Box<dyn SayHelloService> {
    Box::new(PluginSayHello::new(handle))
}

pub struct PluginSayHello {
    id: String,
    handle: Handle,
}

impl PluginSayHello {
    fn new(handle: Handle) -> PluginSayHello {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] Created instance!", id);
        PluginSayHello { id, handle }
    }
}

// #[async_trait]
// impl SayHelloService for PluginSayHello {
//     // this errors with "future cannot be sent between threads safely"
//     async fn say_hello(&self) {
//         // this should enable you to call tokio::sleep
//         // https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#method.enter
//         let _guard = self.handle.enter();
//         println!("[{}] Hello from plugin!", self.id);
//         let _ = tokio::spawn(async move {
//             let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//             println!("sleep 1");
//         })
//         .await;
//     }
// }

#[async_trait]
impl SayHelloService for PluginSayHello {
    async fn say_hello(&self) {
        println!("[{}] Hello from plugin!", self.id);
        let _ = self
            .handle
            .spawn(async move {
                // this errors with "there is no reactor running, must be called from the context of a Tokio 1.x runtime"
                // let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let _ = sleep(Duration::new(1, 0));
                println!("sleep 1");
            })
            .await;
    }
}

impl Drop for PluginSayHello {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", self.id);
    }
}
