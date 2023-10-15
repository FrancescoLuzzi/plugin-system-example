use my_interface::SayHelloService;
use tokio::{self, runtime::Handle};

#[tokio::main]
async fn main() {
    let lib = libloading::Library::new("target/debug/libmy_plugin.so").expect("load library");
    let new_service: libloading::Symbol<fn() -> Box<dyn SayHelloService>> =
        unsafe { lib.get(b"new_service") }.expect("load symbol");
    let service1 = new_service();
    let service2 = new_service();
    // thread '<unnamed>' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime', /home/francesco/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.33.0/src/time/interval.rs:138:26
    // 1/10 calls present this panic, but the execution finishes

    let _ = tokio::join!(
        service1.say_hello(Handle::current()),
        service2.say_hello(Handle::current())
    );
}
