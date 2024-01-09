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
    let uistate = state::UIGraphicsState::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == uistate.window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn main() {
    pollster::block_on(run())
}
