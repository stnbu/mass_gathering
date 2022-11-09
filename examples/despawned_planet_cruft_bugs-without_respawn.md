```
2022-11-09T21:31:42.797817Z DEBUG mass_gathering::physics: Collision of planets:
2022-11-09T21:31:42.797836Z DEBUG mass_gathering::physics:  Major planet 0v0
2022-11-09T21:31:42.797840Z DEBUG mass_gathering::physics:   position: Vec3(-1.13662956e-7, -2.0468725e-8, 0.0021094216)
2022-11-09T21:31:42.797850Z DEBUG mass_gathering::physics:   velocity: Vec3(-9.3372726e-8, -1.2875043e-8, 0.00080747367)
2022-11-09T21:31:42.797855Z DEBUG mass_gathering::physics:   mass: 4188.7905
2022-11-09T21:31:42.797858Z DEBUG mass_gathering::physics:  Minor planet 1v0
2022-11-09T21:31:42.797861Z DEBUG mass_gathering::physics:   position: Vec3(-0.005307735, 0.00051541155, 18.942251)
2022-11-09T21:31:42.797884Z DEBUG mass_gathering::physics:   velocity: Vec3(7.4930664e-5, 0.002591581, -2.0663984)
2022-11-09T21:31:42.797908Z DEBUG mass_gathering::physics:   mass: 3053.6282
2022-11-09T21:31:42.797911Z DEBUG mass_gathering::physics: Directly setting mass of major planet 0v0 to 7242.419
2022-11-09T21:31:42.797915Z DEBUG mass_gathering::physics: The weighted midpoint between planets major=0v0 and minor=1v0 is Vec3(-0.0011189856, 0.00010865081, 3.99393)
2022-11-09T21:31:42.797919Z DEBUG mass_gathering::physics: Sending event: DeltaEvent { entity: 0v0, delta_p: Vec3(-0.001118872, 0.00010867128, 3.9918206), delta_v: Vec3(3.1632455e-5, 0.0010926963, -0.871598), delta_s: 1.2002314 }
2022-11-09T21:31:42.797927Z DEBUG mass_gathering::physics: Signaling despawn request for minor planet 1v0
2022-11-09T21:31:42.797962Z DEBUG mass_gathering::physics: Sending projectile impact event: ProjectileCollisionEvent { planet: 1v0, projectile: 37v4, local_impact_site: Vec3(0.73867047, -1.6557233, 8.8154955) }
2022-11-09T21:31:42.798026Z DEBUG mass_gathering::craft:  Projectile 43v3 traveling toward target on planet 1v0 by delta_p=Vec3(0.03279747, -0.119682476, -3.5291505)
2022-11-09T21:31:42.798033Z DEBUG mass_gathering::craft:  Projectile 37v4 traveling toward target on planet 1v0 by delta_p=Vec3(-0.00019158125, 0.000483346, -0.020217134)
2022-11-09T21:31:42.798038Z DEBUG mass_gathering::craft:  Projectile 50v1 traveling toward target on planet 1v0 by delta_p=Vec3(0.054201134, -0.1214951, -2.983555)
2022-11-09T21:31:42.798047Z DEBUG mass_gathering::physics: RECURSIVELY despawning planet 1v0 and all of its in-flight projectiles
2022-11-09T21:31:42.798054Z DEBUG mass_gathering::craft: Projectile 37v4 impacting planet 1v0, delta_v=Vec3(-0.00013438848, 0.00030123058, -0.001603829)
2022-11-09T21:31:42.798165Z DEBUG mass_gathering::craft: Explosion animation entity 54v0 spawned and now a child of planet 1v0 with local coordiantes Vec3(0.73867047, -1.6557233, 8.8154955)
thread 'main' panicked at 'Entity 1v0 does not exist', /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/world/mod.rs:281:32
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