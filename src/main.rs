
use rodio::source::Source;
struct NoclipController {
    pub transform: *mut Transform,
    pub should_move: bool,
    pub view_x: f32,
}
impl NoclipController {
    pub const IDENTIFIER: &'static str = "NoclipController";
}
impl Component for NoclipController {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, context: &Context) {
        self.transform = Object::get_component(object, Transform::IDENTIFIER).unwrap();
        let str = if self.should_move {
            "shmoovin"
        } else {
            "movement locked"
        };
        (*context.display).gl_window().window().set_title(str);
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let lock_key = (*scene).input.key_state(Key::L);
        if KeyState::held(lock_key) && KeyState::this_tick(lock_key) {
            self.should_move = !self.should_move;
            let str = if self.should_move {
                "shmoovin"
            } else {
                "movement locked"
            };
            (*context.display).gl_window().window().set_title(str);
        }
        if self.should_move {
            let (w, a, s, d) = (
                (*scene).input.key_state(Key::W),
                (*scene).input.key_state(Key::A),
                (*scene).input.key_state(Key::S),
                (*scene).input.key_state(Key::D),
            );
            if KeyState::held(a) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: -(*scene).delta.as_secs_f32(),
                        y: 0.0,
                        z: 0.0,
                    }))
                .into();
            }
            if KeyState::held(d) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: (*scene).delta.as_secs_f32(),
                        y: 0.0,
                        z: 0.0,
                    }))
                .into();
            }
            if KeyState::held(w) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: -(*scene).delta.as_secs_f32(),
                    }))
                .into();
            }
            if KeyState::held(s) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: (*scene).delta.as_secs_f32(),
                    }))
                .into();
            }

            (*self.transform).model = (Matrix4::from((*self.transform).model)
                * Matrix4::from_angle_x(Deg(self.view_x)))
            .into();
            (*self.transform).model = (Matrix4::from((*self.transform).model)
                * Matrix4::from_angle_y(Deg(-(*scene).input.mouse_delta.0 as f32)))
            .into();

            self.view_x += (*scene).input.mouse_delta.1 as f32;
            self.view_x = clamp(self.view_x, -90.0, 90.0);
            (*self.transform).model = (Matrix4::from((*self.transform).model)
                * Matrix4::from_angle_x(Deg(-self.view_x)))
            .into();
        }
        let esc = (*scene).input.key_state(Key::Escape);
        let should_exit = KeyState::held(esc);

        if should_exit {
            return Some(NextScene::Done);
        } else {
            return None;
        }
    }
}

struct Title {
    pub mesh_renderer: *mut MeshRenderer,
}
impl Title {
    pub const IDENTIFIER: &'static str = "Title";
}
impl Component for Title {
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        _scene: *mut Scene,
        _context: &Context,
    ) {
        self.mesh_renderer = Object::get_component(object, MeshRenderer::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        _context: &Context,
    ) -> Option<NextScene> {
        if let UniformValue::Float(offset) =
            (*self.mesh_renderer).uniforms.0.get_mut("offset").unwrap()
        {
            *offset += (*scene).delta.as_secs_f32();
        }
        None
    }
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
struct Splash {
    pub mesh_renderer: *mut MeshRenderer,
}
impl Splash {
    pub const IDENTIFIER: &'static str = "Splash";
}
impl Component for Splash {
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        _scene: *mut Scene,
        _context: &context::Context,
    ) {
        self.mesh_renderer = Object::get_component(object, MeshRenderer::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        _context: &Context,
    ) -> Option<NextScene> {
        if let UniformValue::Float(scale) =
            (*self.mesh_renderer).uniforms.0.get_mut("scale").unwrap()
        {
            *scale += (*scene).delta.as_secs_f32() * PI;
        }
        None
    }
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}

struct Exit {
    pub scene: Option<unsafe fn(&Context) -> Scene>,
    pub transform: *const Transform,
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}
impl Exit {
    pub const IDENTIFIER: &'static str = "Exit";
}
impl Component for Exit {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        _scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let [x, y, z, _] = (*self.transform).model[3];

        if x > self.min.x
            && x < self.max.x
            && y > self.min.y
            && y < self.max.y
            && z > self.min.z
            && z < self.max.z
        {
            if let Some(func) = self.scene {
                Some(NextScene::Another(func(context)))
            } else {
                Some(NextScene::Done)
            }
        } else {
            None
        }
    }
}
enum WackyState {
    Waiting,
    Fading,
    DoingTheFunny,
    Crash,
}
struct TheWackyEntrance {
    pub timer: time::Duration,
    pub player: *const Transform,
    pub mesh_renderer: *mut MeshRenderer,
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
    pub state: WackyState,
    pub current_frame: usize,
    pub frames: Frames<'static>,
}
impl TheWackyEntrance {
    pub fn new(
        player: *const Transform,
        mesh_renderer: *mut MeshRenderer,
        min: Vector3<f32>,
        max: Vector3<f32>,
    ) -> Self {
        Self {
            timer: Duration::ZERO,
            player,
            mesh_renderer,
            min,
            max,
            state : WackyState::Waiting,
            current_frame: 0,
            frames: image::codecs::gif::GifDecoder::new(File::open("data/media/skyrim.gif").unwrap()).unwrap().into_frames(),
        }
    }

    pub const IDENTIFIER: &'static str = "🤡";
    const FADE_TIME: Duration = Duration::from_millis(1_500);
    const VIDEO_FADE_TIME: Duration = Duration::from_secs(3);
    const FPS: Duration = Duration::from_nanos(33_333_333);
    const SWAG: Rect = Rect {
        left: 0,
        bottom: 0,
        width: 256,
        height: 144,
    };
}
use cgmath::SquareMatrix;
use cgmath::vec3;
use glium::Blend;
use glium::BlendingFunction;
use glium::Rect;
use glium::texture::SrgbTexture2d;
use image::{AnimationDecoder, Frames};
use playmotor::component::MeshRenderer;
use playmotor::context;
impl Component for TheWackyEntrance {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let [x, y, z, _] = (*self.player).model[3];
        match self.state {
            WackyState::Waiting => {
                if x > self.min.x
                    && x < self.max.x
                    && y > self.min.y
                    && y < self.max.y
                    && z > self.min.z
                    && z < self.max.z
                {
                    self.state = WackyState::Fading;
                    self.timer = Duration::ZERO;
                    context.sinks["music"].pause();
                    context.sinks["sfx0"].append(asset::load_audio("data/media/start.ogg"));
                    context.sinks["sfx0"].play();
                }

                None
            }
            WackyState::Fading => {
                self.timer += (*scene).delta;

                *(*self.mesh_renderer).uniforms.0.get_mut("opacity").unwrap() =
                    UniformValue::Float(self.timer.as_secs_f32() / Self::FADE_TIME.as_secs_f32());
                if self.timer >= Self::FADE_TIME {
                    self.state = WackyState::DoingTheFunny;
                    self.timer = Duration::ZERO;
                }
                None
            }
            WackyState::DoingTheFunny => {
                self.timer += (*scene).delta;
                context.sinks["trolos"].append(asset::load_audio("data/media/skyrim.ogg"));
                context.sinks["trolos"].play();
                let swag = (self.timer.as_nanos() / Self::FPS.as_nanos()) as usize;
                if swag > self.current_frame {
                    self.current_frame = swag;

                    context.textures["pain"].write(
                        Self::SWAG,
                        glium::texture::RawImage2d::from_raw_rgba_reversed(
                            self.frames.next().unwrap().unwrap().buffer().as_raw(),
                            (256, 144),
                        ),
                    );
                }
                *(*self.mesh_renderer)
                    .uniforms
                    .0
                    .get_mut("video_opacity")
                    .unwrap() = UniformValue::Float(clamp(
                    self.timer.as_secs_f32() / Self::VIDEO_FADE_TIME.as_secs_f32(),
                    0.0,
                    1.0,
                ));

                None
            }
            WackyState::Crash => Some(NextScene::Done),
        }
    }
}

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
    let mut scene = Scene::new_without_view(
        &*context.display,
        vec![
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: (Matrix4::from_angle_x(Deg(22.5))
                            * Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0)))
                        .into(),
                        ..Default::default()
                    }),
                    Box::new(component::MeshRenderer {
                        mesh: &context.meshes["title"],
                        prog: &context.shader_programs["title"],

                        uniforms: DynamicUniforms(
                            collection! {"offset" => UniformValue::Float(0.0_f32)},
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
                    Box::new(Title {
                        mesh_renderer: ptr::null_mut(),
                    }),
                ],
            },
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: Into::<[[f32; 4]; 4]>::into(
                            Matrix4::from_angle_x(Deg(22.5))
                                * Matrix4::from_translation(Vector3::new(1.25, 0.5, -2.9)),
                        ),
                        ..Default::default()
                    }),
                    Box::new(component::MeshRenderer {
                        mesh: &context.meshes["splash"],
                        prog: &context.shader_programs["splash"],

                        uniforms: DynamicUniforms(collection! {
                            "scale" => UniformValue::Float(1.0_f32),
                            "model2" => UniformValue::Mat4(Into::<[[f32;4];4]>::into(Matrix4::from_angle_x(Deg(22.5)) * Matrix4::from_translation(Vector3::new(1.25, 0.5, -2.9)) * Matrix4::from_scale(0.5))),
                        }),
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
                    Box::new(Splash {
                        mesh_renderer: ptr::null_mut(),
                    }),
                ],
            },
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: Matrix4::identity().into(),
                        ..Default::default()
                    }),
                    Box::new(NoclipController {
                        should_move: true,
                        transform: ptr::null_mut(),
                        view_x: 0.0,
                    }),
                    Box::new(component::Camera {
                        transform: ptr::null_mut(),
                    }),
                ],
            },
        ],
        (0.0, 0.0, 0.01, 1.0),
    );
    scene.objects.push(Object {
        components: vec![
            Box::new(component::Transform {
                model: Matrix4::from_translation(vec3(1.0, -1.0, -4.0)).into(),
                ..Default::default()
            }),
            Box::new(component::MeshRenderer {
                mesh: &context.meshes["exit"],
                prog: &context.shader_programs["door"],
                uniforms: DynamicUniforms(collection! {}),
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
            Box::new(Exit {
                scene: None,
                transform: Object::get_component(&scene.objects[2] as *const Object, "Transform")
                    .unwrap(),
                min: Vector3 {
                    x: 0.5,
                    y: -2.0,
                    z: -4.5,
                },
                max: Vector3 {
                    x: 1.5,
                    y: 2.0,
                    z: -3.5,
                },
            }),
        ],
    });
    let swag = context as *const Context;
    scene.objects.push(Object {
        components: vec![
            Box::new(component::Transform {
                model: Matrix4::from_translation(vec3(-1.0, -1.0, -4.0)).into(),
                ..Default::default()
            }),
            Box::new(component::MeshRenderer {
                mesh: &context.meshes["start"],
                prog: &context.shader_programs["door"],
                uniforms: DynamicUniforms(collection! {}),
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
            Box::new(component::MeshRenderer {
                mesh: &context.meshes["plane"],
                prog: &context.shader_programs["trolos"],
                uniforms: DynamicUniforms(collection! {
                    "opacity" => UniformValue::Float(0.0),
                    "video" => UniformValue::SrgbTexture2d(&(*swag).textures["pain"], None),
                    "video_opacity" => UniformValue::Float(0.0),
                }),
                draw_parameters: glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    blend: Blend {
                        color: BlendingFunction::Addition {
                            source: glium::LinearBlendingFactor::SourceAlpha,
                            destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                        },
                        alpha: BlendingFunction::Addition {
                            source: glium::LinearBlendingFactor::SourceAlpha,
                            destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                        },
                        constant_value: (0.0, 0.0, 0.0, 0.0),
                    },
                    ..Default::default()
                },
                transform: ptr::null_mut(),
            }),
        ],
    });

    let bruh = Box::new(TheWackyEntrance::new(
        Object::get_component(&scene.objects[2] as *const Object, "Transform").unwrap(),
        &*(scene.objects[4].components[2]) as *const dyn component::Component
            as *mut dyn component::Component as *mut MeshRenderer,
        vec3(-1.5, -2.0, -4.5),
        vec3(-0.5, 2.0, -3.5),
    ));
    scene.objects[4].components.push(bruh);

    return scene;
}
use std::f32::consts::PI;
use std::fs::File;
use std::time;
use std::ptr;
use std::time::Duration;

use cgmath::Deg;
use cgmath::num_traits::clamp;
use cgmath::{Matrix4, Vector3};
use glium::glutin;
use glium::glutin::event::MouseScrollDelta;
use glium::glutin::event::VirtualKeyCode as Key;
use glium::uniforms::UniformValue;
use glium::{glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder}, Display};
use playmotor::asset;
use playmotor::component::Component;
use playmotor::component::Transform;
use playmotor::input::KeyState;
use playmotor::scene::NextScene;
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
                //"moon-monkey" => todo!(),
                "pain" => SrgbTexture2d::empty(&display, 256,144).unwrap(),
            },
            collection! {
                "monkey" => asset::load_model("data/meshes/monkey.obj", &display),
                "title" => asset::load_model("data/meshes/title.obj", &display),
                "splash" => asset::load_model("data/meshes/splash.obj", &display),
                "start" => asset::load_model("data/meshes/start.obj", &display),
                "exit" => asset::load_model("data/meshes/exit.obj", &display),
                "plane" => asset::load_model("data/meshes/plane.obj", &display),
            },
            collection!(
                "monkey" => asset::load_program("data/shaders/monkey", &display),
                "title" => asset::load_program("data/shaders/title", &display),
                "splash" => asset::load_program("data/shaders/splash", &display),
                "door" => asset::load_program("data/shaders/start", &display),
                "trolos" => asset::load_program("data/shaders/trolos", &display),
            ),
            ["music", "sfx0", "trolos"],
        );
        context.sinks["music"].append(asset::load_audio("data/media/title.ogg").repeat_infinite());
        context.sinks["music"].play();

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
