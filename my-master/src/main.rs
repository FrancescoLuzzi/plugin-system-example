use my_interface::SayHelloService;
use tokio::{self, runtime::Handle};

#[tokio::main]
async fn main() {
    let lib = libloading::Library::new("target/debug/libmy_plugin.so").expect("load library");
    let new_service: libloading::Symbol<fn(Handle) -> Box<dyn SayHelloService>> =
        unsafe { lib.get(b"new_service") }.expect("load symbol");
    let service1 = new_service(Handle::current());
    let service2 = new_service(Handle::current());
    let _ = tokio::join!(service1.say_hello(), service2.say_hello());
}
