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

    let mut uistate = state::UIGraphicsState::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == uistate.window.id() => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(window_id) if window_id == uistate.window.id() => {
                match uistate.render() {
                    Ok(_) => {}
                    Err(E) => eprintln!("{:?}", E),
                }
            }

            Event::MainEventsCleared => {
                uistate.window.request_redraw();
            }
            _ => (),
        }
    });
}

fn main() {
    pollster::block_on(run())
}
