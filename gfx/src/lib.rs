use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, keyboard::{PhysicalKey, KeyCode},
};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let w_id = window.id();

    let _ = event_loop.run(move |event, _| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == w_id => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {}, //event_loop.set_control_flow(ControlFlow::Wait),
            _ => {}
        },
        _ => {}
    });

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_min_inner_size(Some(PhysicalSize::new(450, 400)));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("draw-area")?;
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
}