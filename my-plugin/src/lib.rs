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

#[async_trait]
impl SayHelloService for PluginSayHello {
    async fn say_hello(&self) {
        let id = self.id.clone();
        let _ = self
            .handle
            .spawn_blocking(move || {
                println!("[{}] Hello from plugin!", id);
                let _ = sleep(Duration::new(1, 0));
                println!("slept 1");
                println!("[{}] Hello again from plugin!", id);
            })
            .await;
    }
}

impl Drop for PluginSayHello {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", self.id);
    }
}
