use std::{ops::Deref, rc::Rc};

use gfx_glyph::ab_glyph::FontRef;
use wgpu_text::{
    glyph_brush::{Section, Text},
    BrushBuilder, TextBrush,
};
use winit::window::Window;

pub trait Drawable {
    fn draw(&mut self, render_pass: &mut wgpu::RenderPass);
}

pub struct UIGraphicsState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    config: wgpu::SurfaceConfiguration,
    window: Rc<Window>,
}

impl UIGraphicsState {
    pub async fn new(window: Rc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window.deref()) }.unwrap();

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

    pub fn window(&self) -> &Window {
        return &self.window;
    }

    pub fn surface(&self) -> &wgpu::Surface {
        return &self.surface;
    }

    pub fn device(&self) -> &wgpu::Device {
        return &self.device;
    }

    pub fn queue(&self) -> &wgpu::Queue {
        return &self.queue;
    }
}

pub fn render<T: Drawable>(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    object: &mut T,
) -> Result<(), wgpu::SurfaceError> {
    let output = surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        object.draw(&mut render_pass)
    }

    queue.submit([encoder.finish()]);
    output.present();

    Ok(())
}

pub struct CustomText<'a> {
    state: &'a UIGraphicsState,
    brush: TextBrush<FontRef<'a>>,
    sections: Vec<Section<'a>>,
}

impl<'a> CustomText<'a> {
    pub fn new(state: &'a UIGraphicsState) -> Self {
        let font: &[u8] = include_bytes!("fonts/SourceCodePro-Regular.ttf");
        let brush = BrushBuilder::using_font_bytes(font).unwrap().build(
            &state.device,
            state.size.width,
            state.size.height,
            state.config.format,
        );
        return Self {
            state,
            brush,
            sections: vec![],
        };
    }

    pub fn write(&mut self, text: &'a str) {
        let s = Section::default().add_text(Text::new(text).with_scale(14.));
        self.sections.push(s)
    }
}

impl<'a> Drawable for CustomText<'a> {
    fn draw(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.brush
            .queue(
                &self.state.device,
                &self.state.queue,
                (&self.sections[..]).into(),
            )
            .unwrap();
        self.brush.draw(render_pass);
    }
}
