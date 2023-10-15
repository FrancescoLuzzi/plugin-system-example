use async_trait::async_trait;
use my_interface::{SayHelloService, WithRuntime};
use tokio::{self, runtime::Handle};

#[no_mangle]
pub fn new_service() -> Box<dyn SayHelloService> {
    Box::new(PluginSayHello::new())
}

pub struct PluginSayHello {
    id: String,
}

impl PluginSayHello {
    fn new() -> PluginSayHello {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] Created instance!", id);
        PluginSayHello { id }
    }
}

/// https://stackoverflow.com/questions/77294605/library-plugin-manager-in-rust-is-it-even-doable-right-now#comment136267977_77295025
#[async_trait]
impl SayHelloService for PluginSayHello {
    async fn say_hello(&self, handle: Handle) {
        WithRuntime::new(handle, async move {
            println!("[{}] Hello from plugin!", self.id);
            // internal code of reqwest just crashes
            let body = reqwest::get("https://api.ipify.org")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("body = {:?}", body);
        })
        .await;
    }
}

impl Drop for PluginSayHello {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", self.id);
    }
}
