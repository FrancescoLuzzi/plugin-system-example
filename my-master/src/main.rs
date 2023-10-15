use my_interface::SayHelloService;
use tokio::{self, runtime::Handle};

type GetHandleFn = fn() -> Handle;
type PluginInitFn = fn(GetHandleFn) -> Box<dyn SayHelloService>;

#[tokio::main]
async fn main() {
    fn get_current_handle() -> Handle {
        Handle::current()
    }
    let lib = libloading::Library::new("target/debug/libmy_plugin.so").expect("load library");
    let new_service: libloading::Symbol<PluginInitFn> =
        unsafe { lib.get(b"new_service") }.expect("load symbol");
    {
        // 1/10 run a thread panics with "there is no reactor running, must be called from the context of a Tokio 1.x runtime"
        // but the execution seems to finish correctly (I doubt about it)
        let service1 = new_service(get_current_handle);
        let service2 = new_service(get_current_handle);
        let _ = tokio::join!(service1.say_hello(), service2.say_hello());
    }
}
