use alloc::{boxed::Box, sync::Arc};
use core::{pin::Pin, task::Context};

use futures_util::{
    task::{waker, ArcWake},
    Future,
};
use spin::Mutex;

use super::executor::Executor;

//

pub struct Task {
    executor: Arc<Executor>,
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    // future: Mutex<Pin<dyn Future<Output = ()>>>,
}

//

impl Task {
    pub fn spawn(executor: Arc<Executor>, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Self::_new(executor.clone(), fut));
        executor.add_task(task);
    }

    pub fn poll(self: Arc<Self>) {
        let waker = waker(self.clone());
        let mut ctx = Context::from_waker(&waker);

        let Some(mut future) = self
            .future
            .try_lock() else {
                // another CPU is already working on this task
                return;
            };

        _ = future.as_mut().poll(&mut ctx);
    }

    pub fn schedule(self: &Arc<Self>) {
        self.executor.add_task(self.clone());
    }

    fn _new(executor: Arc<Executor>, fut: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            future: Mutex::new(Box::pin(fut)),
            executor,
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}
