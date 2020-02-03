

#[cfg(feature = "fixture")]
mod test_util;
mod util;

#[cfg(feature = "fixture")]
pub use async_test_derive::test_async;

pub use util::sleep;
pub use task::*;

mod task {


    use std::future::Future;

    use async_std::task::JoinHandle;
    use async_std::task;
    use log::trace;

    use super::sleep;

    /// run future and wait forever
    /// this is typically used in the server
    pub fn run<F>(spawn_closure: F)
    where
        F: Future<Output = ()> + Send + 'static 
    {   
        task::block_on(spawn_closure);
}

    /// run future and wait forever
    /// this is typically used in the server
    pub fn main<F>(spawn_closure: F)
    where
        F: Future<Output = ()> + Send + 'static 
    {   
        use std::time::Duration;

        task::block_on(async{
            spawn_closure.await;
            // do infinite loop for now
            loop {
                sleep(Duration::from_secs(3600)).await;
            }
        });
    }


    pub fn spawn<F,T>(future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + 'static + Send, 
        T: Send + 'static
    {
        trace!("spawning future");
        task::spawn(future)
    }

    pub fn spawn_blocking<F, T>(future: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
    {
        trace!("spawning blocking");
        task::spawn_blocking(future)
    }


    /// same as async async std block on
    pub fn run_block_on<F,T>(f:F) -> T
        where F: Future<Output = T>
    {
        task::block_on(f)
    } 

}

#[cfg(test)]
mod test {

    use lazy_static::lazy_static;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::{thread, time};

    use super::run;
    use super::spawn;

    #[test]
    fn test_spawn3() {
        lazy_static! {
            static ref COUNTER: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        }

        assert_eq!(*COUNTER.lock().unwrap(), 0);

        let ft = async {
            thread::sleep(time::Duration::from_millis(100));
            *COUNTER.lock().unwrap() = 10;
        };

        run(async {
            let join_handle = spawn(ft);
            join_handle.await;
        });

        assert_eq!(*COUNTER.lock().unwrap(), 10);
    }

    /*
    // this is sample code to show how to keep test goging
    //#[test]
    fn test_sleep() {
       

        let ft = async {
            for _ in 0..100 {
                println!("sleeping");
                super::sleep(time::Duration::from_millis(1000)).await;
            }
           
        };

        run(async {
            let join_handle = spawn(ft);
            join_handle.await;
        });
    
    }
    */

    /*
    use std::future::Future;
    use std::task::Context;
    use std::task::Poll;
    use std::pin::Pin;
    use std::io;


    use async_std::task::spawn_blocking;

    struct BlockingFuture {
    }

    impl Future for BlockingFuture {

        type Output = io::Result<()>;

        fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
            
            println!("start poll");
            
            spawn_blocking(move || { 
                println!("start sleeping");
                thread::sleep(time::Duration::from_millis(100));
                println!("wake up from sleeping");
            });
            
            Poll::Pending
        }

    }

    //#[test]
    fn test_block_spawning() {

        run(async {
            let block = BlockingFuture{};
            block.await;
        });        

    }
    */


}
