// -----------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Tree {
    Leaf,
    Branch(usize, Vec<Tree>),
}

impl Default for Tree {
    fn default() -> Self {
        Self::Leaf
    }
}

impl Tree {
    fn big(branching_factor: usize, depth: usize) -> Self {
        if depth == 0 {
            Self::Leaf
        } else {
            let t = Self::big(branching_factor, depth - 1);
            let mut branches = vec![];
            for i in 0..branching_factor {
                branches.push(t.clone());
            }
            Self::Branch(depth, branches)
        }
    }
}

pub type Path = Vec<usize>;

// -----------------------------------------------------------------------------

fn snoc(path_init: &Path, step: usize) -> Path {
    let mut path = path_init.clone();
    path.push(step);
    path
}

// -----------------------------------------------------------------------------

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Content {
    root: Tree,
    cursor: Path,
}

impl Content {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        use Tree::*;

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // // Load previous app state (if any).
        // // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // } else {
        //     Default::default()
        // }

        Self {
            root: Tree::big(2, 8),
            cursor: vec![],
        }
    }

    fn render_root(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| self.render_tree(ui, vec![], &self.root.clone()));
    }

    fn render_tree(&mut self, ui: &mut egui::Ui, path: Path, t: &Tree) {
        match t {
            Tree::Leaf => {}
            Tree::Branch(x, ts) => {
                egui::Frame::default()
                    .inner_margin(12)
                    .stroke(if self.cursor == path {
                        egui::Stroke::new(1.0, egui::Color32::RED)
                    } else {
                        egui::Stroke::new(1.0, egui::Color32::GRAY)
                    })
                    .show(ui, |ui| {
                        let button = ui.button(format!("{x}"));
                        if button.clicked() {
                            self.cursor = path.clone()
                        }
                        for (step, t) in ts.iter().enumerate() {
                            self.render_tree(ui, snoc(&path, step), t);
                        }
                    });
            }
        };
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("tree_nav");

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    self.render_root(ui);
                });

            if ctx.input(|i| i.key_pressed(egui::Key::A)) {
                println!("key pressed: A");
            }
        });
    }
}
