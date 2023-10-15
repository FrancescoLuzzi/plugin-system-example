use async_trait::async_trait;
use my_interface::{SayHelloService, WithRuntime};
use tokio::{self, runtime::Handle};

#[no_mangle]
pub fn new_service(get_handle: fn() -> Handle) -> Box<dyn SayHelloService> {
    Box::new(PluginSayHello::new(get_handle))
}

pub struct PluginSayHello {
    id: String,
    get_handle: fn() -> Handle,
}

impl PluginSayHello {
    fn new(get_handle: fn() -> Handle) -> PluginSayHello {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] Created instance!", id);
        PluginSayHello { id, get_handle }
    }
}

/// https://stackoverflow.com/questions/77294605/library-plugin-manager-in-rust-is-it-even-doable-right-now#comment136267977_77295025
#[async_trait]
impl SayHelloService for PluginSayHello {
    async fn say_hello(&self) {
        WithRuntime::new(self.get_handle, async move {
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
