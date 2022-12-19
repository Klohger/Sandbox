use core::time;
use std::{f32::consts::PI, fs::File, ptr};

use cgmath::{num_traits::clamp, Deg, Matrix4, Vector3};
use glium::{glutin::event::VirtualKeyCode as Key, uniforms::UniformValue, Rect};
use image::{AnimationDecoder, Frames};
use playmotor::{
    asset,
    component::{self, Component},
    context::Context,
    input::KeyState,
    object::Object,
    scene::{NextScene, Scene},
};

pub struct NoclipController {
    pub transform: *mut component::Transform,
    pub should_move: bool,
    pub view_x: f32,
}
impl NoclipController {
    pub const IDENTIFIER: &'static str = "NoclipController";
}
impl Default for NoclipController {
    fn default() -> Self {
        Self {
            transform: ptr::null_mut(),
            should_move: true,
            view_x: 0.0,
        }
    }
}
impl Component for NoclipController {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, context: &Context) {
        self.transform = Object::get_component(object, component::Transform::IDENTIFIER).unwrap();
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

pub struct Title {
    pub mesh_renderer: *mut component::MeshRenderer,
}
impl Title {
    pub const IDENTIFIER: &'static str = "Title";
}
impl Component for Title {
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, _context: &Context) {
        self.mesh_renderer =
            Object::get_component(object, component::MeshRenderer::IDENTIFIER).unwrap();
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

pub struct Splash {
    pub mesh_renderer: *mut component::MeshRenderer,
}
impl Splash {
    pub const IDENTIFIER: &'static str = "Splash";
}
impl Component for Splash {
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, _context: &Context) {
        self.mesh_renderer =
            Object::get_component(object, component::MeshRenderer::IDENTIFIER).unwrap();
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

pub struct Exit {
    pub scene: Option<unsafe fn(&Context) -> Scene>,
    pub transform: *const component::Transform,
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

pub enum WackyState {
    Waiting,
    Fading,
    DoingTheFunny,
    Crash,
}
pub struct TheWackyEntrance {
    pub timer: time::Duration,
    pub player: *const component::Transform,
    pub mesh_renderer: *mut component::MeshRenderer,
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
    pub state: WackyState,
    pub current_frame: usize,
    pub frames: Frames<'static>,
}
impl TheWackyEntrance {
    pub fn new(
        player: *const component::Transform,
        mesh_renderer: *mut component::MeshRenderer,
        min: Vector3<f32>,
        max: Vector3<f32>,
    ) -> Self {
        Self {
            timer: time::Duration::ZERO,
            player,
            mesh_renderer,
            min,
            max,
            state: WackyState::Waiting,
            current_frame: 0,
            frames: image::codecs::gif::GifDecoder::new(
                File::open("data/media/skyrim.gif").unwrap(),
            )
            .unwrap()
            .into_frames(),
        }
    }

    pub const IDENTIFIER: &'static str = "🤡";
    const FADE_TIME: time::Duration = time::Duration::from_millis(1_500);
    const VIDEO_FADE_TIME: time::Duration = time::Duration::from_secs(3);
    const FPS: time::Duration = time::Duration::from_nanos(33_333_333);
    const SWAG: Rect = Rect {
        left: 0,
        bottom: 0,
        width: 256,
        height: 144,
    };
}
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
                    self.timer = time::Duration::ZERO;
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
                    self.timer = time::Duration::ZERO;
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

                    context.textures["skyrim"].write(
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
