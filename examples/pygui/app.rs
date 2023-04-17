use anyhow::{anyhow, Result};

use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    EventPump,
    event::Event,
    video::{GLProfile, Window},
};
use pyo3::prelude::*;
use python_gui;

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

#[pyclass(unsendable)]
struct Application {
    window: Window,
    gl_context: sdl2::video::GLContext,
    platform: SdlPlatform,
    event_pump: EventPump,
    renderer: AutoRenderer,
    imgui: Context,
}

impl Application {
    fn rust_window(ui: &imgui::Ui) {
        ui.window("Rust Window")
            .build(|| {
                ui.text("Hello from Rust!");
            });
    }
}

#[pymethods]
impl Application {
    #[new]
    pub fn new() -> Result<Self> {
        let sdl = sdl2::init().map_err(|e| anyhow!("SDL init: {}", e))?;
        let video_subsystem = sdl.video().map_err(|e| anyhow!("SDL video: {}", e))?;

        /* hint SDL to initialize an OpenGL 3.3 core profile context */
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);

        /* create a new window, be sure to call opengl method on the builder when using glow! */
        let window = video_subsystem
            .window("Hello imgui-rs!", 1280, 720)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        /* create a new OpenGL context and make it current */
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        /* enable vsync to cap framerate */
        window.subsystem().gl_set_swap_interval(1).unwrap();

        /* create new glow and imgui contexts */
        let gl = glow_context(&window);

        /* create context */
        let mut imgui = Context::create();

        /* disable creation of files on disc */
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        /* setup platform and renderer, and fonts to imgui */
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        /* create platform and renderer */
        let platform = SdlPlatform::init(&mut imgui);
        let renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

        /* start main loop */
        let event_pump = sdl.event_pump().unwrap();

        Ok(Application {
            window,
            gl_context,
            platform,
            event_pump,
            renderer,
            imgui,
        })
    }

    pub fn prepare_frame(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            self.platform.handle_event(&mut self.imgui, &event);

            if let Event::Quit { .. } = event {
                return false;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        self.platform.prepare_frame(&mut self.imgui, &self.window, &self.event_pump);
        let ui = self.imgui.new_frame();
        /* create imgui UI here */
        //ui.show_demo_window(&mut true);
        Self::rust_window(&ui);
        true
    }

    pub fn render_frame(&mut self) {
        let draw_data = self.imgui.render();
        unsafe { self.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        self.renderer.render(draw_data).unwrap();
        self.window.gl_swap_window();
    }
}


#[pymodule]
fn app(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(python_gui::get(py)?)?;
    m.add_class::<Application>()?;
    Ok(())
}
