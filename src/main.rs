#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod shader;
pub mod vector2;

use eframe::{
    egui::{self, Rect, Sense},
    egui_wgpu,
};

use vector2::*;

fn main() -> eframe::Result {
    // Viewport options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true),
        multisampling: 1,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    // Start the program
    eframe::run_native(
        "Fractal Viewer",
        options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}


/// Struct containing all application state info
struct Application {
    camera: CameraInfo,
    max_iter: i32,
    exponent: f32,
    fractal_type: FractalType,
    shading_type: ShadingType,
    color_scheme: ColorScheme,
    palette_speed: f32,
    julia: bool,
    julia_pos: Vector2d
}

/// Contains a cosine color palette for the shader
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorScheme {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub c: [f32; 3],
    pub d: [f32; 3],
}

impl ColorScheme {
    pub const fn new(a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3]) -> Self {
        Self { a, b, c, d }
    }
    // A few color palettes from here: https://iquilezles.org/articles/palettes/
    const RAINBOW: Self = Self::new([0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [1.0, 1.0, 1.0], [0.00, 0.33, 0.67]);
    const EARTH: Self = Self::new([0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [1.0, 1.0, 1.0], [0.00, 0.10, 0.20]);
    const SKY: Self = Self::new([0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [1.0, 1.0, 1.0], [0.30, 0.20, 0.20]);
    const MIDDAY: Self = Self::new([0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [1.0, 1.0, 0.5], [0.80, 0.90, 0.30]);
    const MIDNIGHTAMBER: Self = Self::new([0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [1.0, 0.7, 0.4], [0.00, 0.15, 0.20]);
    const SUNSET: Self = Self::new([0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [2.0, 1.0, 0.0], [0.50, 0.20, 0.25]);
    const CRIMSON: Self = Self::new([0.8, 0.5, 0.4], [0.2, 0.4, 0.2], [2.0, 1.0, 1.0], [0.00, 0.25, 0.25]);
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum ShadingType {
    Normal,
    Smooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum FractalType {
    Mandelbrot,
    BurningShip,
    Tricorn,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraInfo {
    pub pos: Vector2d,
    pub zoom: f64,
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            pos: Vector2::default(),
            zoom: 2.1,
        }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Get the WGPU render state
        let wgpu_render_state = cc
            .wgpu_render_state
            .as_ref()
            .expect("You need to run eframe with the wgpu backend");

        // compile and link the shader program
        shader::init(wgpu_render_state);
        Self { // Setup the initial settings
            camera: CameraInfo::default(),
            max_iter: 1024,
            exponent: 2.0,
            fractal_type: FractalType::Mandelbrot,
            shading_type: ShadingType::Smooth,
            color_scheme: ColorScheme::MIDNIGHTAMBER,
            palette_speed: 0.05,
            julia: false,
            julia_pos: Vector2d::default(),
        }
    }

    /// Custom WGPU shader painting and input processing
    fn custom_painting(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let rect = self.inputs(ui, ctx);

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            shader::RenderCallback {
                pos: (self.camera.pos.x as f32, self.camera.pos.y as f32).into(),
                zoom: self.camera.zoom as f32,
                _p0: Default::default(),
                resolution: rect.size().into(),
                offset: rect.min.into(),
                max_iter: self.max_iter,
                exponent: self.exponent,
                fractal_type: self.fractal_type as u32,
                shading_type: self.shading_type as u32,
                color_scheme: self.color_scheme.into(),
                palette_speed: self.palette_speed,
                julia: self.julia as u32,
                julia_pos: (self.julia_pos.x as f32, self.julia_pos.y as f32).into(),
            },
        ));
    }

    /// Input processing
    fn inputs(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> Rect {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

        // Get the scale of the viewport relative to world coordinates
        let viewport_scale = rect.width().min(rect.height());

        // Get the mouse position in normalized device coordinates (NDC)
        let mouse_pos = ctx.input(|i| i.pointer.latest_pos().unwrap_or_default());
        let mouse_ndc = Vector2d::new(
            ((mouse_pos.x - rect.center().x) / viewport_scale * 2.0) as f64,
            ((mouse_pos.y - rect.center().y) / viewport_scale * 2.0) as f64,
        );

        // Calculate the world position under the mouse before zooming
        let world_before_zoom = self.camera.pos + mouse_ndc * self.camera.zoom;

        // Zooming
        ctx.input(|i| {
            self.camera.zoom *= 1.01_f64.powf(-i.raw_scroll_delta.y as f64);

            if i.zoom_delta() != 1.0 {
                self.camera.zoom *= i.zoom_delta() as f64;
            }
        });
        // Calculate the world position under the mouse after zooming
        let world_after_zoom = self.camera.pos + mouse_ndc * self.camera.zoom;

        // Adjust camera position to keep the world position under the mouse constant
        self.camera.pos += world_after_zoom - world_before_zoom;

        // Drag handling
        if response.dragged_by(egui::PointerButton::Primary) {
            let drag_motion: Vector2f = response.drag_motion().into();
            let mut drag_delta: Vector2d = Vector2d::new(drag_motion.x as f64, drag_motion.y as f64);
            drag_delta /= viewport_scale as f64;
            drag_delta *= self.camera.zoom * 2.0;
    
            self.camera.pos += drag_delta;
        }
        if response.secondary_clicked() || response.dragged_by(egui::PointerButton::Secondary) {
            let click_position: Vector2f = response.interact_pointer_pos().unwrap().into();
            self.julia_pos = (Vector2d::new(click_position.x as f64, click_position.y as f64)
                               - Vector2d::new(rect.center().x as f64, rect.center().y as f64))
                               / viewport_scale as f64 * 2.0 * self.camera.zoom + self.camera.pos;
            
        }
        rect
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for if the F11 key is pressed
        // ctx.send_viewport_cmd crashes the program when it is called inside of ctx.input
        let mut do_fullscreen = false;
        ctx.input(|i| {
            if i.key_pressed(egui::Key::F11) {
                do_fullscreen = true;
            }
        });
        if do_fullscreen {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                !ctx.input(|i| i.viewport().fullscreen.unwrap_or(false)),
            ))
        }

        // Render the settings panel
        egui::SidePanel::right("settings_panel")
            .resizable(true)
            .show(ctx, |ui| {
                // Double the size of the sliders (the default is 100)
                ui.spacing_mut().slider_width = 200.0;
                ui.vertical_centered(|ui| {
                    if ui.button("Reset camera").clicked() {
                        self.camera = CameraInfo::default();
                    }
                });
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    ui.label("Max iterations: ");
                    ui.add(
                        egui::Slider::new(&mut self.max_iter, 1..=4096)
                            .logarithmic(true)
                            .clamp_to_range(false)
                            .smart_aim(true),
                    );

                    ui.label("Exponent: ");
                    ui.add(
                        egui::Slider::new(&mut self.exponent, 0.0..=6.0)
                            .clamp_to_range(false)
                            .smart_aim(true),
                    );

                    ui.label("Fractal: ");
                    ui.horizontal_wrapped(|ui| {
                        ui.radio_value(
                            &mut self.fractal_type,
                            FractalType::Mandelbrot,
                            "Mandelbrot",
                        );
                        ui.radio_value(
                            &mut self.fractal_type,
                            FractalType::BurningShip,
                            "Burning Ship",
                        );
                        ui.radio_value(&mut self.fractal_type, FractalType::Tricorn, "Tricorn");
                    });

                    ui.separator();

                    ui.checkbox(&mut self.julia, "Julia set");
                    ui.label("Julia position: ");
                    ui.add(
                        egui::Slider::new(&mut self.julia_pos.x, -2.0..=2.0)
                            .clamp_to_range(false)
                            .smart_aim(true)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.julia_pos.y, -2.0..=2.0)
                            .clamp_to_range(false)
                            .smart_aim(true)
                            .prefix("y: "),
                    );
                    ui.label("Right click on the fractal to set the location of the julia set.");

                    ui.separator();

                    ui.label("Shading Type: ");
                    ui.horizontal_wrapped(|ui| {
                        ui.radio_value(&mut self.shading_type, ShadingType::Normal, "Normal");
                        ui.radio_value(&mut self.shading_type, ShadingType::Smooth, "Smooth");
                    });

                    ui.label("Palette Speed: ");
                    ui.add(
                        egui::Slider::new(&mut self.palette_speed, 0.0..=1.0)
                            .logarithmic(true)
                            .clamp_to_range(false)
                            .smart_aim(true),
                    );

                    ui.separator();

                    ui.label("Color Scheme: ");
                    ui.horizontal_wrapped(|ui| {
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::EARTH,
                            "Earth",
                        );
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::SKY,
                            "Sky",
                        );
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::CRIMSON,
                            "Crimson",
                        );
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::MIDNIGHTAMBER,
                            "Midnight Amber",
                        );
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::RAINBOW,
                            "Rainbow",
                        );
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::SUNSET,
                            "Sunset",
                        );
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::MIDDAY,
                            "Midday",
                        );
                    });
                });
            });
        
        // Display the main shader and position info
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0]
                    .vertical_centered(|ui| ui.label(format!("position: {}", self.camera.pos)));

                columns[1].vertical_centered(|ui| ui.label(format!("zoom: {}", self.camera.zoom)));
            });

            // Make a canvas for the shader
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui, ctx);
            });
        });
    }
}
