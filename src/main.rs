
#[macro_export]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

const REFRESH_RATES: [time::Duration; 2] = [
    time::Duration::from_nanos(16_666_667),
    time::Duration::from_nanos(15_625_000),
];

unsafe fn title(context: &Context) -> Scene {
    let scene = Scene::new(
        &*context.display,
        vec![
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: Matrix4::from_translation(Vector3::new(0.0,0.0,-1.0)).into(),
                        ..Default::default()
                    }),
                    Box::new(component::MeshRenderer {
                        mesh: &context.meshes["monkey"],
                        prog: &context.shader_programs["monkey"],

                        uniforms: DynamicUniforms(
                            collection! {},
                        ),
                        draw_parameters: glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        transform: ptr::null_mut(),
                    }),
                ],
            },
        ],
        Matrix4::identity().into(),
        Scene::default().clear_color,
        
    );
    return scene;
}
use std::time;
use std::ptr;

use cgmath::{Matrix4, Vector3, SquareMatrix};
use glium::glutin;
use glium::glutin::event::MouseScrollDelta;
use glium::{glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder}, Display};
use playmotor::asset;
use playmotor::{context::Context, scene::{Scene, self}, object::Object, component::{self, DynamicUniforms}};

fn main() {
    unsafe {
        let events_loop = EventLoop::new();
        let display = Display::new(
            WindowBuilder::new().with_maximized(true),
            ContextBuilder::new()
                .with_depth_buffer(24)
                .with_double_buffer(Some(true))
                .with_multisampling(1),
            &events_loop,
        )
        .unwrap();
        let context = Context::new(
            &display,
            collection! {
            },
            collection! {
                "monkey" => asset::load_model("monkey.obj", &display)
            },
            collection!{
                "monkey" => asset::load_program("monkey", &display)
            },
            [],
        );

        let mut scene = title(&context);
        Scene::init(&mut scene as *mut Scene, &context);

        let mut should_exit = false;
        let refresh_rate = REFRESH_RATES[1];
        events_loop.run(move |event, _target, control_flow| {
            let now = time::Instant::now();

            if now >= scene.next_frame_instant {
                Scene::draw(&mut scene as *mut Scene, &context);

                match Scene::update(&mut scene as *mut Scene, &context, now, refresh_rate) {
                    Some(next_scene) => match next_scene {
                        scene::NextScene::Another(new_scene) => {
                            scene = new_scene;
                            Scene::init(&mut scene as *mut Scene, &context);
                        }
                        scene::NextScene::Done => should_exit = true,
                    },
                    None => (),
                }
            }

            if should_exit {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            } else {
                match event {
                    glutin::event::Event::WindowEvent {
                        window_id: _,
                        event,
                    } => match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                        glutin::event::WindowEvent::Resized(size) => {
                            scene.proj = cgmath::perspective(
                                cgmath::Deg(90.0),
                                size.width as f32 / size.height as f32,
                                0.05,
                                100.0,
                            )
                            .into();
                        }
                        _ => (),
                    },
                    glutin::event::Event::RedrawRequested(_) => {
                        Scene::draw(&mut scene as *mut Scene, &context);
                    }
                    glutin::event::Event::DeviceEvent {
                        device_id: _,
                        event,
                    } => match event {
                        glutin::event::DeviceEvent::Key(key) => {
                            scene.input.poll_keys(key);
                        }
                        glutin::event::DeviceEvent::MouseMotion { delta } => {
                            scene.input.poll_mouse(delta);
                        }
                        glutin::event::DeviceEvent::MouseWheel { delta } => match delta {
                            MouseScrollDelta::LineDelta(x, y) => scene.input.poll_scroll((x, y)),
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                }

                control_flow.set_wait_until(scene.next_frame_instant);
            }
        });
    }
}
