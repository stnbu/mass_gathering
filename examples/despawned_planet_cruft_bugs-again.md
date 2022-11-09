```
2022-11-09T21:27:39.514099Z DEBUG mass_gathering::craft:  Projectile 54v9 traveling toward target on planet 59v1 by delta_p=Vec3(0.0008877352, -0.006766582, 0.22020645)
2022-11-09T21:27:39.514105Z DEBUG mass_gathering::craft:  Projectile 46v9 traveling toward target on planet 59v1 by delta_p=Vec3(0.033397116, 0.10162656, -3.59199)
2022-11-09T21:27:39.514110Z DEBUG mass_gathering::craft:  Projectile 28v1 traveling toward target on planet 59v1 by delta_p=Vec3(0.049246904, 0.08111128, -2.9527636)
2022-11-09T21:27:39.514120Z DEBUG mass_gathering::physics: Collision of planets:
2022-11-09T21:27:39.514132Z DEBUG mass_gathering::physics:  Major planet 9v0
2022-11-09T21:27:39.514203Z DEBUG mass_gathering::physics:   position: Vec3(0.017013669, -0.0497969, 0.19021091)
2022-11-09T21:27:39.514216Z DEBUG mass_gathering::physics:   velocity: Vec3(0.003405953, -0.00889378, -0.87058973)
2022-11-09T21:27:39.514221Z DEBUG mass_gathering::physics:   mass: 7242.419
2022-11-09T21:27:39.514225Z DEBUG mass_gathering::physics:  Minor planet 59v1
2022-11-09T21:27:39.514229Z DEBUG mass_gathering::physics:   position: Vec3(0.013194252, -0.0009753129, 21.180796)
2022-11-09T21:27:39.514233Z DEBUG mass_gathering::physics:   velocity: Vec3(0.0062417723, -0.00071351946, -2.0421803)
2022-11-09T21:27:39.514238Z DEBUG mass_gathering::physics:   mass: 3053.6282
2022-11-09T21:27:39.514242Z DEBUG mass_gathering::physics: Directly setting mass of major planet 9v0 to 10296.047
2022-11-09T21:27:39.514337Z DEBUG mass_gathering::physics: The weighted midpoint between planets major=9v0 and minor=59v1 is Vec3(0.007940448, -0.017658634, 3.2078264)
2022-11-09T21:27:39.514343Z DEBUG mass_gathering::physics: Sending event: DeltaEvent { entity: 9v0, delta_p: Vec3(-0.009073221, 0.032138266, 3.0176156), delta_v: Vec3(0.00084105483, 0.0024261223, -0.34747338), delta_s: 1.124421 }
2022-11-09T21:27:39.514350Z DEBUG mass_gathering::physics: Signaling despawn request for minor planet 59v1
2022-11-09T21:27:39.514373Z DEBUG mass_gathering::physics: RECURSIVELY despawning planet 59v1 and all of its in-flight projectiles
2022-11-09T21:27:39.514382Z DEBUG mass_gathering::craft: Projectile 54v9 impacting planet 59v1, delta_v=Vec3(4.3035154e-5, -0.00034074226, -0.0016009716)
2022-11-09T21:27:39.514468Z DEBUG mass_gathering::craft: Explosion animation entity 27v1 spawned and now a child of planet 59v1 with local coordiantes Vec3(-0.23654407, 1.8729004, 8.79979)
thread 'main' panicked at 'Entity 59v1 does not exist', /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/world/mod.rs:281:32
stack backtrace:
   0: rust_begin_unwind
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/core/src/panicking.rs:142:14
   2: bevy_ecs::world::World::entity_mut::{{closure}}
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/world/mod.rs:281:32
   3: core::option::Option<T>::unwrap_or_else
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/core/src/option.rs:825:21
   4: bevy_ecs::world::World::entity_mut
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/world/mod.rs:280:9
   5: <bevy_hierarchy::child_builder::AddChild as bevy_ecs::system::commands::Command>::write
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_hierarchy-0.8.1/src/child_builder.rs:117:26
   6: bevy_ecs::system::commands::command_queue::CommandQueue::push::write_command
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/commands/command_queue.rs:42:13
   7: bevy_ecs::system::commands::command_queue::CommandQueue::apply
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/commands/command_queue.rs:96:17
   8: <bevy_ecs::system::commands::command_queue::CommandQueue as bevy_ecs::system::system_param::SystemParamState>::apply
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/system_param.rs:531:9
   9: <(P0,P1,P2,P3,P4,P5) as bevy_ecs::system::system_param::SystemParamState>::apply
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/system_param.rs:1320:19
  10: <bevy_ecs::system::function_system::FunctionSystem<In,Out,Param,Marker,F> as bevy_ecs::system::system::System>::apply_buffers
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/function_system.rs:411:9
  11: <bevy_ecs::schedule::stage::SystemStage as bevy_ecs::schedule::stage::Stage>::run
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/schedule/stage.rs:909:29
  12: bevy_ecs::schedule::Schedule::run_once
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/schedule/mod.rs:342:13
  13: bevy_app::app::App::update
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.8.1/src/app.rs:119:9
  14: <winit::platform_impl::platform::app_state::EventLoopHandler<T> as winit::platform_impl::platform::app_state::EventHandler>::handle_nonuser_event::{{closure}}
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/app_state.rs:101:13
  15: winit::platform_impl::platform::app_state::EventLoopHandler<T>::with_callback
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/app_state.rs:79:13
  16: <winit::platform_impl::platform::app_state::EventLoopHandler<T> as winit::platform_impl::platform::app_state::EventHandler>::handle_nonuser_event
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/app_state.rs:100:9
  17: winit::platform_impl::platform::app_state::Handler::handle_nonuser_event
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/app_state.rs:206:21
  18: winit::platform_impl::platform::app_state::AppState::cleared
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/app_state.rs:387:9
  19: winit::platform_impl::platform::observer::control_flow_end_handler::{{closure}}
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/observer.rs:180:21
  20: winit::platform_impl::platform::observer::control_flow_handler::{{closure}}
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/observer.rs:142:57
  21: std::panicking::try::do_call
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/std/src/panicking.rs:492:40
  22: std::panicking::try
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/std/src/panicking.rs:456:19
  23: std::panic::catch_unwind
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/std/src/panic.rs:137:14
  24: winit::platform_impl::platform::event_loop::stop_app_on_panic
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/event_loop.rs:229:11
  25: winit::platform_impl::platform::observer::control_flow_handler
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/observer.rs:142:5
  26: winit::platform_impl::platform::observer::control_flow_end_handler
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/observer.rs:175:9
  27: <unknown>
  28: <unknown>
  29: <unknown>
  30: <unknown>
  31: <unknown>
  32: <unknown>
  33: <unknown>
  34: <unknown>
  35: <unknown>
  36: <unknown>
  37: <() as objc::message::MessageArguments>::invoke
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/objc-0.2.7/src/message/mod.rs:128:17
  38: objc::message::platform::send_unverified
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/objc-0.2.7/src/message/apple/mod.rs:27:9
  39: objc::message::send_message
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/objc-0.2.7/src/message/mod.rs:178:5
  40: winit::platform_impl::platform::event_loop::EventLoop<T>::run_return::{{closure}}
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/event_loop.rs:188:22
  41: objc::rc::autorelease::autoreleasepool
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/objc-0.2.7/src/rc/autorelease.rs:29:5
  42: winit::platform_impl::platform::event_loop::EventLoop<T>::run_return
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/event_loop.rs:178:9
  43: winit::platform_impl::platform::event_loop::EventLoop<T>::run
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/platform_impl/macos/event_loop.rs:157:9
  44: winit::event_loop::EventLoop<T>::run
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.26.1/src/event_loop.rs:154:9
  45: bevy_winit::run
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_winit-0.8.1/src/lib.rs:240:5
  46: bevy_winit::winit_runner_with
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_winit-0.8.1/src/lib.rs:663:9
  47: bevy_winit::winit_runner
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_winit-0.8.1/src/lib.rs:280:5
  48: core::ops::function::Fn::call
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/core/src/ops/function.rs:77:5
  49: <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/alloc/src/boxed.rs:1965:9
  50: bevy_app::app::App::run
             at /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.8.1/src/app.rs:135:9
  51: despawned_planet_cruft_bugs::main
             at ./examples/despawned_planet_cruft_bugs.rs:6:5
  52: core::ops::function::FnOnce::call_once
             at /rustc/43347397f7c5ca9a670a3bb3890c7187e24a52ab/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```