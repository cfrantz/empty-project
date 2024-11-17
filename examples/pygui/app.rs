use anyhow::{anyhow, Result};

use glow::HasContext;
use imgui::{ConfigFlags, Context};
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use pyo3::prelude::*;
use python_gui;
use sdl2::{
    event::Event,
    video::{GLProfile, SwapInterval, Window},
    EventPump,
};
use send_wrapper::SendWrapper;

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

#[pyclass(unsendable)]
struct Application {
    window: SendWrapper<Window>,
    _gl_context: sdl2::video::GLContext,
    platform: SdlPlatform,
    event_pump: EventPump,
    renderer: AutoRenderer,
    imgui: Context,
    dpi: f32,
}

#[pyclass(unsendable)]
#[repr(transparent)]
struct UiContext {
    pub ui: &'static imgui::Ui,
}

impl UiContext {
    pub fn new(ui: &imgui::Ui) -> Self {
        unsafe {
            Self {
                ui: std::mem::transmute(ui),
            }
        }
    }
}

impl Application {}

#[pymethods]
impl Application {
    #[new]
    pub fn new() -> Result<Self> {
        let sdl = sdl2::init().map_err(|e| anyhow!("SDL init: {e}"))?;
        let video_subsystem = sdl.video().map_err(|e| anyhow!("SDL video: {e}"))?;

        let (dpi, _hdpi, _vdpi) = video_subsystem
            .display_dpi(0)
            .map_err(|e| anyhow!("SDL display_dpi: {e}"))?;

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
        let _gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&_gl_context).unwrap();

        /* enable vsync to cap framerate */
        video_subsystem
            .gl_set_swap_interval(SwapInterval::VSync)
            .map_err(|e| anyhow!("SDL swap_interval: {e}"))?;

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

        imgui.io_mut().config_flags |= ConfigFlags::DOCKING_ENABLE;

        /* create platform and renderer */
        let platform = SdlPlatform::init(&mut imgui);
        let renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

        /* start main loop */
        let event_pump = sdl.event_pump().unwrap();

        Ok(Application {
            window: SendWrapper::new(window),
            _gl_context,
            platform,
            event_pump,
            renderer,
            imgui,
            dpi,
        })
    }

    pub fn set_scale(&mut self, scale: f32) {
        let mut scale = scale;
        if scale == 0.0 {
            scale = self.dpi / 96.0;
        }
        let style = self.imgui.style_mut();
        style.scale_all_sizes(scale);
        self.imgui.io_mut().font_global_scale = scale;
    }

    pub fn prepare_frame(&mut self) -> Option<UiContext> {
        for event in self.event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            self.platform.handle_event(&mut self.imgui, &event);

            if let Event::Quit { .. } = event {
                return None;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        self.platform
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = UiContext::new(self.imgui.new_frame());

        /* create imgui UI here */
        //ui.show_demo_window(&mut true);
        //Self::rust_window(&ui);
        Some(ui)
    }

    pub fn render_frame(&mut self, py: Python<'_>) {
        let draw_data = self.imgui.render();
        unsafe { self.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        if draw_data.draw_lists_count() > 0 {
            self.renderer.render(draw_data).unwrap();
        }
        py.allow_threads(|| self.window.gl_swap_window());
    }

    fn rust_fragment(&self, ctx: &mut UiContext) {
        ctx.ui.text("This is a rust fragment!");
    }

    fn rust_window(&self, ctx: &mut UiContext) {
        ctx.ui.window("Rust Window").build(|| {
            ctx.ui.text("Hello from Rust!");
        });
    }
}

#[pymodule(name = "libapp")]
fn app(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python_gui::as_submodule_of(m)?;
    m.add_class::<Application>()?;
    Ok(())
}
