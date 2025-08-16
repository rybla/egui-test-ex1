use std::rc::Rc;

// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn is_empty(&self) -> bool {
        match self {
            List::Nil => true,
            _ => false,
        }
    }
}

pub type Path = List<usize>;

impl<A> Default for List<A> {
    fn default() -> Self {
        Self::Nil
    }
}

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
            for _i in 0..branching_factor {
                branches.push(t.clone());
            }
            Self::Branch(depth, branches)
        }
    }

    fn get_subtree(&self, path: &Path) -> Option<&Self> {
        match path {
            List::Nil => Option::Some(self),
            List::Cons(h, path) => match self {
                Tree::Leaf => None,
                Tree::Branch(_, ts) => match ts.get(*h) {
                    None => None,
                    Some(t) => t.get_subtree(path),
                },
            },
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Content {
    root: Tree,
    cursor: Rc<Path>,
}

impl Content {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
            cursor: Rc::new(List::Nil),
        }
    }

    pub fn get_cursor_tree(&self) -> &Tree {
        let cursor = self.cursor.clone();
        let root = self.root.clone();
        todo!()
    }

    // fn move_forward(&mut self) {}
    // fn move_backward(&mut self) {}

    pub fn move_right(&mut self) {}

    pub fn move_left(&mut self) {}

    pub fn move_down(&mut self) {}

    pub fn move_up(&mut self) {}

    fn render_root(&mut self, ui: &mut egui::Ui) {
        let cursor = self.cursor.clone();
        let root = self.root.clone();
        ui.scope(|ui| self.render_tree(ui, Option::Some(cursor), Rc::new(List::Nil), &root));
    }

    fn render_tree(
        &mut self,
        ui: &mut egui::Ui,
        opt_cursor: Option<Rc<Path>>,
        path: Rc<Path>,
        t: &Tree,
    ) {
        match t {
            Tree::Leaf => {}
            Tree::Branch(x, ts) => {
                let is_cursor = match &opt_cursor {
                    Some(cursor) => cursor.is_empty(),
                    None => false,
                };
                egui::Frame::default()
                    .inner_margin(12)
                    .stroke(if is_cursor {
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
                            self.render_tree(
                                ui,
                                opt_cursor.clone(),
                                Rc::new(List::Cons(step, path.clone())),
                                t,
                            );
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

            if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                self.move_left();
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                self.move_right();
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.move_up();
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.move_down();
            }
        });
    }
}
