mod lines;
mod bindings;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;
use lines::PixelDrawer;
use shared::{GameState, GraphicsContext};

#[derive(Copy, Clone)]
struct Color(u8, u8, u8);

struct App {
    window: Option<Window>,
    input: WinitInputHelper,
    pixels: Option<Pixels>,
    game_state: GameState,
    last_frame: std::time::Instant,
}

const WHITE: Color = Color(255, 255, 255);

impl App {
    fn render(&mut self) {
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        let keys = bindings::keys(&self.input);
        self.game_state.keys(dt, keys);
        let pixels = match &mut self.pixels {
            Some(pixels) => pixels,
            None => return,
        };
        let t = pixels.context().texture_extent;
        let (width, height) = (t.width as u32, t.height as u32);
        let mut drawer = PixelDrawer::new(pixels);
        drawer.clear(WHITE);

        let mut graphics_ctx = GraphicsContext {
            drawer: &mut drawer,
            width,
            height,
        };
        self.game_state.draw_screen(&mut graphics_ctx);
        pixels.render().unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &window);

        let pixels = Pixels::new(320, 240, surface).expect("Pixels init failed");

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let mut render = false;
        if let Some(window) = self.window.as_mut() {
            match event {
                WindowEvent::RedrawRequested => render = true,
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        return;
                    }
                    if let Some(pixels) = &mut self.pixels {
                        pixels.resize_surface(size.width, size.height).unwrap();
                        pixels.resize_buffer(size.width, size.height).unwrap();
                    }
                }

                _ => {}
            }
            // Feed the helper
            if self.input.process_window_event(&event) {
                window.request_redraw();
            }
        }

        if render {
            self.render();
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        self.input.process_device_event(&event);
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        if self.input.key_released(KeyCode::KeyQ)
            || self.input.close_requested()
            || self.input.destroyed()
        {
            event_loop.exit();
            return;
        }

        if let Some(window) = &self.window {
            // For continuous rendering:
            window.request_redraw();
        }
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App {
        window: None,
        pixels: None,
        input: WinitInputHelper::new(),
        last_frame: std::time::Instant::now(),
        game_state: GameState::new(),
    }).unwrap();
    Ok(())
}
