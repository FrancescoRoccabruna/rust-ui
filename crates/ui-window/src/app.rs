use std::{
    num::NonZeroU32,
    rc::Rc,
};

use softbuffer::{Context, Surface};
use ui_core::{Constraints, Widget};
use ui_render::{
    Color, DrawCommand, Rect, Renderer, Size, SoftwareRenderer,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct WindowApp {
    context: Option<Context<winit::event_loop::OwnedDisplayHandle>>,
    window: Option<Rc<Window>>,
    surface: Option<
        Surface<
            winit::event_loop::OwnedDisplayHandle,
            Rc<Window>,
        >
    >,
    renderer: Option<SoftwareRenderer>,
    root: Box<dyn Widget>,
}

impl WindowApp {
    pub fn new<W>(root: W) -> Self
    where
        W: Widget + 'static, 
    {
        Self {
            context: None,
            window: None,
            surface: None,
            renderer: None,
            root: Box::new(root),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;

        self.context = Some(
            Context::new(event_loop.owned_display_handle())?
        );

        event_loop.run_app(&mut self)?;

        Ok(())
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("rust-ui")
                )
                .unwrap()
        );

        let context = self
            .context
            .as_ref()
            .unwrap();

        let surface = Surface::new(
            context,
            window.clone(),
        )
        .unwrap();

        let size = window.inner_size();

        self.renderer = Some(
            SoftwareRenderer::new(
                size.width,
                size.height,
            )
        );

        self.surface = Some(surface);
        self.window = Some(window);

        self.window
            .as_ref()
            .unwrap()
            .request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(_) => {
                window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                self.redraw();
            }

            _ => {}
        }
    }
}

impl WindowApp {
    fn redraw(&mut self) {
        let Some(window) = &self.window else {
            return;
        };

        let Some(surface) = &mut self.surface else {
            return;
        };

        let size = window.inner_size();

        if size.width == 0 || size.height == 0 {
            return;
        }

        surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();

        let renderer = self
            .renderer
            .as_mut()
            .unwrap();

        renderer.resize(
            size.width,
            size.height,
        );

        let constraints = Constraints::loose(
            Size::new(
                size.width as f32,
                size.height as f32,
            )
        );

        let root_size = self.root.measure(constraints);

        let root_rect = Rect::new(
            0.0,
            0.0,
            root_size.width,
            root_size.height,
        );

        let mut commands = Vec::new();

        self.root.draw(
            root_rect,
            &mut commands,
        );

        renderer.clear(
            Color::rgb(255, 255, 255)
        );

        renderer
            .render(&commands)
            .unwrap();

        let mut buffer =
            surface.buffer_mut().unwrap();

        for (destination, source) in buffer
            .iter_mut()
            .zip(renderer.pixels().chunks_exact(4))
        {
            let r = source[0] as u32;
            let g = source[1] as u32;
            let b = source[2] as u32;

            *destination =
                (r << 16)
                | (g << 8)
                | b;
        }

        window.pre_present_notify();

        buffer.present().unwrap();
    }
}