use winit::window::Window;
use wgpu_text::{BrushBuilder, TextBrush};
use wgpu_text::glyph_brush{Section, Text};

pub trait Drawable {
    fn draw(&self, render_pass: &wgpu::RenderPass);
}

pub struct UIGraphicsState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    config: wgpu::SurfaceConfiguration,
    pub window: Window,
}

impl UIGraphicsState {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_capability = surface.get_capabilities(&adapter);
        let surface_format = surface_capability
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capability.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capability.present_modes[0],
            alpha_mode: surface_capability.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }

    pub fn render(&mut self, object: &impl Drawable) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            object.draw(&render_pass)
        }

        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }
}

struct CustomText {
    state: &UIGraphicsState,
    brush: TextBrush,
    section: Section,
}

impl CustomText {
    pub fn new(state: &UIGraphicsState, text: String) -> Self {
        let font: &[u8] - include_bytes!("fonts/SourceCodePro-Regular.ttf");
        let brush = BrushBuilder::using_font_bytes.unwrap().build(state.device, state.size.width, state.size.height, state.config.format);
        let section = Section::default().add_text(Text::new(text)).with_scale(font_size)
        return Self { state, brush, section };
    }
}

impl Drawable for CustomText {
    fn draw(&self, render_pass: &wgpu::RenderPass) {
        self.brush.queue(&self.state.device, &self.state.queue, vec![&self.section])
        self.brush.draw(&render_pass)
    }
}
