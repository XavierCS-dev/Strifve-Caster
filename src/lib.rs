use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod engine;
use engine::render_data::{self, RenderData};

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Learn WGPU")
        .build(&event_loop)
        .unwrap();
    window.set_resizable(false);
    let mut render_data = RenderData::new(window).await;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } if window_id == render_data.window().id() => if !render_data.input(event) { *control_flow = ControlFlow::Exit,}
            Event::RedrawRequested(window_id) if window_id == render_data.window().id() => {
                render_data.update();
                render_data.render().unwrap();
            }
            Event::RedrawEventsCleared => {
                render_data.window().request_redraw();
            }
            _ => (),
        }
    })
}
