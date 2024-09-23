use eframe::egui;

use crate::{Application, CameraInfo, ColorScheme, FractalType, ShadingType};

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
                        ui.radio_value(&mut self.color_scheme, ColorScheme::EARTH, "Earth");
                        ui.radio_value(&mut self.color_scheme, ColorScheme::SKY, "Sky");
                        ui.radio_value(&mut self.color_scheme, ColorScheme::CRIMSON, "Crimson");
                        ui.radio_value(
                            &mut self.color_scheme,
                            ColorScheme::MIDNIGHTAMBER,
                            "Midnight Amber",
                        );
                        ui.radio_value(&mut self.color_scheme, ColorScheme::RAINBOW, "Rainbow");
                        ui.radio_value(&mut self.color_scheme, ColorScheme::SUNSET, "Sunset");
                        ui.radio_value(&mut self.color_scheme, ColorScheme::MIDDAY, "Midday");
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
