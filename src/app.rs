use egui::{
    plot::{Legend, Line, Plot},
    Ui,
};
use itertools_num::linspace;
use probability::prelude::{Continuous, Distribution};

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub enum Distributions {
    Uniform(Uniform),
    Gaussian(Gaussian),
    Gamma(Gamma),
    LogNormal(LogNormal),
}
#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Uniform {
    name: String,
    lower_bound: f64,
    upper_bound: f64,
}

impl Default for Uniform {
    fn default() -> Self {
        Self {
            name: "Uniform".to_string(),
            lower_bound: -1.,
            upper_bound: 1.,
        }
    }
}

impl Uniform {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Uniform");
        ui.add(
            egui::DragValue::new(&mut self.lower_bound)
                .clamp_range(f64::NEG_INFINITY..=self.upper_bound)
                .prefix("Lower bound: "),
        );
        ui.add(
            egui::DragValue::new(&mut self.upper_bound)
                .clamp_range(self.lower_bound..=f64::INFINITY)
                .prefix("Upper bound: "),
        );
    }

    pub fn calculate_pdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution =
            probability::distribution::Uniform::new(self.lower_bound, self.upper_bound);
        x.into_iter()
            .map(|x| [x, distribution.density(x)])
            .collect()
    }

    pub fn calculate_cdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution =
            probability::distribution::Uniform::new(self.lower_bound, self.upper_bound);
        x.into_iter()
            .map(|x| [x, distribution.distribution(x)])
            .collect()
    }

    pub fn plot(&self, ui: &mut Ui) {
        let x: Vec<f64> = linspace::<f64>(-10., 10., 1000).collect();
        let data_pdf = self.calculate_pdf(x.clone());
        let line_pdf = Line::new(data_pdf).name("pdf");
        let data_cdf = self.calculate_cdf(x);
        let line_cdf = Line::new(data_cdf).name("cdf");
        Plot::new("uniform_plot")
            .view_aspect(2.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(line_pdf);
                plot_ui.line(line_cdf);
            });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Gaussian {
    name: String,
    mu: f64,
    sigma: f64,
}

impl Default for Gaussian {
    fn default() -> Self {
        Self {
            name: "Uniform".to_string(),
            mu: 0.,
            sigma: 1.,
        }
    }
}

impl Gaussian {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Gaussian");
        ui.add(
            egui::DragValue::new(&mut self.mu)
                // .clamp_range(f64::NEG_INFINITY..=f64::INFINITY)
                .prefix("mu: "),
        );
        ui.add(
            egui::DragValue::new(&mut self.sigma)
                .clamp_range(0. ..=f64::INFINITY)
                .prefix("sigma: "),
        );
    }

    pub fn calculate_pdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution = probability::distribution::Gaussian::new(self.mu, self.sigma);
        x.into_iter()
            .map(|x| [x, distribution.density(x)])
            .collect()
    }

    pub fn calculate_cdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution = probability::distribution::Gaussian::new(self.mu, self.sigma);
        x.into_iter()
            .map(|x| [x, distribution.distribution(x)])
            .collect()
    }

    pub fn plot(&self, ui: &mut Ui) {
        let x: Vec<f64> = linspace::<f64>(-10., 10., 1000).collect();
        let data_pdf = self.calculate_pdf(x.clone());
        let line_pdf = Line::new(data_pdf).name("pdf");
        let data_cdf = self.calculate_cdf(x);
        let line_cdf = Line::new(data_cdf).name("cdf");
        Plot::new("gaussian_plot")
            .view_aspect(2.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(line_pdf);
                plot_ui.line(line_cdf);
            });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Gamma {
    name: String,
    k: f64,
    theta: f64,
}

impl Default for Gamma {
    fn default() -> Self {
        Self {
            name: "Gamma".to_string(),
            k: 2.,
            theta: 1.,
        }
    }
}

impl Gamma {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Gamma");
        ui.add(
            egui::DragValue::new(&mut self.k)
                .clamp_range(0. ..=f64::INFINITY)
                .prefix("k: "),
        );
        ui.add(
            egui::DragValue::new(&mut self.theta)
                .clamp_range(0. ..=f64::INFINITY)
                .prefix("theta: "),
        );
    }

    pub fn calculate_pdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution = probability::distribution::Gamma::new(self.k, self.theta);
        x.into_iter()
            .map(|x| [x, distribution.density(x)])
            .collect()
    }

    pub fn calculate_cdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution = probability::distribution::Gamma::new(self.k, self.theta);
        x.into_iter()
            .map(|x| [x, distribution.distribution(x)])
            .collect()
    }

    pub fn plot(&self, ui: &mut Ui) {
        let x: Vec<f64> = linspace::<f64>(-10., 10., 1000).collect();
        let data_pdf = self.calculate_pdf(x.clone());
        let line_pdf = Line::new(data_pdf).name("pdf");
        let data_cdf = self.calculate_cdf(x);
        let line_cdf = Line::new(data_cdf).name("cdf");
        Plot::new("gamma_plot")
            .view_aspect(2.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(line_pdf);
                plot_ui.line(line_cdf);
            });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct LogNormal {
    name: String,
    mu: f64,
    sigma: f64,
}

impl Default for LogNormal {
    fn default() -> Self {
        Self {
            name: "LogNormal".to_string(),
            mu: 0.,
            sigma: 1.,
        }
    }
}

impl LogNormal {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Uniform");
        ui.add(egui::DragValue::new(&mut self.mu).prefix("mu: "));
        ui.add(
            egui::DragValue::new(&mut self.sigma)
                .clamp_range(0. ..=f64::INFINITY)
                .prefix("sigma: "),
        );
    }

    pub fn calculate_pdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution = probability::distribution::Lognormal::new(self.mu, self.sigma);
        x.into_iter()
            .map(|x| [x, distribution.density(x)])
            .collect()
    }

    pub fn calculate_cdf(&self, x: Vec<f64>) -> Vec<[f64; 2]> {
        // let mut source = source::default(42);
        let distribution = probability::distribution::Lognormal::new(self.mu, self.sigma);
        x.into_iter()
            .map(|x| [x, distribution.distribution(x)])
            .collect()
    }

    pub fn plot(&self, ui: &mut Ui) {
        let x: Vec<f64> = linspace::<f64>(-10., 10., 1000).collect();
        let data_pdf = self.calculate_pdf(x.clone());
        let line_pdf = Line::new(data_pdf).name("pdf");
        let data_cdf = self.calculate_cdf(x);
        let line_cdf = Line::new(data_cdf).name("cdf");
        Plot::new("uniform_plot")
            .view_aspect(2.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(line_pdf);
                plot_ui.line(line_cdf);
            });
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VizProbApp {
    // Example stuff:
    distribution: Distributions,
}

impl Default for VizProbApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            distribution: Distributions::Uniform(Uniform::default()),
        }
    }
}

impl VizProbApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for VizProbApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { distribution } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ComboBox::from_label("Distribution")
                .selected_text(format!(
                    "{}",
                    match distribution {
                        Distributions::Uniform(dist) => &dist.name,
                        Distributions::Gaussian(dist) => &dist.name,
                        Distributions::Gamma(dist) => &dist.name,
                        Distributions::LogNormal(dist) => &dist.name,
                    }
                ))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        distribution,
                        Distributions::Uniform(Uniform::default()),
                        "Uniform",
                    );
                    ui.selectable_value(
                        distribution,
                        Distributions::Gaussian(Gaussian::default()),
                        "Gaussian",
                    );
                    ui.selectable_value(
                        distribution,
                        Distributions::Gamma(Gamma::default()),
                        "Gamma",
                    );
                    ui.selectable_value(
                        distribution,
                        Distributions::LogNormal(LogNormal::default()),
                        "LogNormal",
                    );
                });
            match distribution {
                Distributions::Uniform(dist) => dist.ui(ui),
                Distributions::Gaussian(dist) => dist.ui(ui),
                Distributions::Gamma(dist) => dist.ui(ui),
                Distributions::LogNormal(dist) => dist.ui(ui),
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            match distribution {
                Distributions::Uniform(dist) => dist.plot(ui),
                Distributions::Gaussian(dist) => dist.plot(ui),
                Distributions::Gamma(dist) => dist.plot(ui),
                Distributions::LogNormal(dist) => dist.plot(ui),
            }
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
