use std::sync::{atomic::AtomicI32, Arc};

use piccolo::{Callback, Closure, Executor, Lua};

fn main() {
    let shared_data = Arc::new(AtomicI32::new(0));

    let shared_data_clone = shared_data.clone();

    let mut lua = Lua::core();

    lua.load_io();

    lua.enter(|ctx| {
        let callback = Callback::from_fn(&ctx, move |ctx, executor, stack| {
            shared_data_clone.store(240, std::sync::atomic::Ordering::Relaxed);

            Ok(piccolo::CallbackReturn::Return)
        });

        ctx.globals().set(ctx, "data_store_test", callback).unwrap();
    });

    let executor = lua.enter(|ctx| {
        let closure = Closure::load(ctx, None, &br#"data_store_test(); print("Hello from Lua!")"#[..]).unwrap();

        ctx.stash(Executor::start(
            ctx,
            piccolo::Function::Closure(closure),
            (),
        ))
    });

    lua.execute::<()>(&executor).unwrap();

    dbg!(shared_data.load(std::sync::atomic::Ordering::Relaxed));
}
