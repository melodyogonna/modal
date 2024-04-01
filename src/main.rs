use std::rc::Rc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod state;

async fn run() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Modal Editor")
        .build(&event_loop)
        .unwrap();
    let window = Rc::new(window);

    let uistate = state::UIGraphicsState::new(window.clone()).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == uistate.window().id() => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(window_id) if window_id == uistate.window().id() => {
                let mut text = state::CustomText::new(&uistate);
                text.write("Some word");
                match state::render(
                    uistate.surface(),
                    uistate.device(),
                    uistate.queue(),
                    &mut text,
                ) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            Event::MainEventsCleared => {
                uistate.window().request_redraw();
            }
            _ => (),
        }
    });
}

fn main() {
    pollster::block_on(run())
}
