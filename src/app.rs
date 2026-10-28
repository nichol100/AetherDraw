// Top-level application object.
//
// eframe calls `update()` once per frame. We keep the whole app state here:
// document (canvas + layers), active tool, theme, workspaces, audio engine, etc.

use crate::canvas::document::Document;
use crate::canvas::viewport::Viewport2D;
use crate::history::History;
use crate::platform::paths::AppPaths;
use crate::ui::theme::ThemeManager;
use crate::ui::workspaces::Workspace;
use egui::{Context, Ui};

pub struct AetherApp {
    /// All canvas + layer data.
    pub document: Document,
    /// 2D view transform (zoom/pan/rotation).
    pub viewport: Viewport2D,
    /// Active workspace (Paint / Animate / Model / Physics).
    pub workspace: Workspace,
    /// Theme manager (runtime-switchable themes).
    pub themes: ThemeManager,
    /// OS-specific paths (config, cache, ...).
    pub paths: AppPaths,
    /// Whether to show the demo/test panels.
    pub show_debug: bool,
    /// Undo/redo history.
    pub history: History,
}

impl AetherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load or create the default theme tokens from disk.
        let paths = AppPaths::new();
        let themes = ThemeManager::new(&paths, &cc.egui_ctx);

        // Start with a blank document.
        let document = Document::new(1920, 1080);

        Self {
            document,
            viewport: Viewport2D::default(),
            workspace: Workspace::Paint,
            themes,
            paths,
            show_debug: false,
            history: History::default(),
        }
    }
}

impl eframe::App for AetherApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 1. Apply the selected theme to egui visuals.
        self.themes.apply(ctx);

        // 2. Top menu bar.
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            self.draw_menu(ui, ctx);
        });

        // 3. Workspace switcher (Paint / Animate / Model / Physics).
        egui::TopBottomPanel::top("workspace_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for ws in Workspace::ALL {
                    if ui
                        .selectable_label(self.workspace == *ws, ws.label())
                        .clicked()
                    {
                        self.workspace = *ws;
                    }
                }
            });
        });

        // 4. Side panels depend on workspace.
        egui::SidePanel::left("tools_panel")
            .default_width(64.0)
            .show(ctx, |ui| {
                ui.label("Tools");
                // Tool buttons are drawn here (stubs for now).
                ui.separator();
                if ui.button("Brush").clicked() {}
                if ui.button("Eraser").clicked() {}
                if ui.button("Move").clicked() {}
            });

        egui::SidePanel::right("layers_panel")
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Layers");
                self.document.draw_layer_list(ui);
            });

        // 5. Bottom panel: timeline (shown in Animate workspace).
        if self.workspace == Workspace::Animate {
            egui::TopBottomPanel::bottom("timeline")
                .default_height(180.0)
                .show(ctx, |ui| {
                    ui.label("Timeline (frame-by-frame cels, keyframes, audio tracks)");
                });
        }

        // 6. Central canvas area.
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_canvas(ui, ctx);
        });

        // Request a repaint so animation/playback stays smooth.
        ctx.request_repaint();
    }
}

impl AetherApp {
    fn draw_menu(&mut self, ui: &mut Ui, ctx: &Context) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.document = Document::new(1920, 1080);
                    ui.close_menu();
                }
                if ui.button("Open...").clicked() {
                    ui.close_menu();
                }
                if ui.button("Save").clicked() {
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    // eframe handles window close; nothing extra needed here.
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    self.history.undo(&mut self.document);
                    ui.close_menu();
                }
                if ui.button("Redo").clicked() {
                    self.history.redo(&mut self.document);
                    ui.close_menu();
                }
                ui.separator();
                ui.label(format!(
                    "Undo: {}  Redo: {}",
                    self.history.can_undo() as i32,
                    self.history.can_redo() as i32
                ));
            });

            ui.menu_button("View", |ui| {
                ui.menu_button("Theme", |ui| {
                    // Runtime theme switching.
                    for name in self.themes.available() {
                        if ui.button(name).clicked() {
                            self.themes.select(name);
                            ui.close_menu();
                        }
                    }
                });
                if ui
                    .checkbox(&mut self.show_debug, "Debug panels")
                    .clicked()
                {}
            });

            ui.menu_button("Help", |ui| {
                if ui.button("About AetherDraw").clicked() {
                    ui.close_menu();
                }
            });

            // Keep ctx used (avoids warnings in this scaffold).
            let _ = ctx;
        });
    }

    fn draw_canvas(&mut self, ui: &mut Ui, ctx: &Context) {
        // Allocate the whole central area as a painter.
        let (rect, response) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        // Fill background with the theme canvas color.
        ui.painter().rect_filled(rect, 0.0, self.themes.canvas_bg());

        // Draw the document (layers composited) into the canvas rect.
        // For Phase 1 we just draw a placeholder grid + document size label.
        self.viewport.draw_grid(ui.painter(), rect);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("Canvas {}x{}", self.document.width, self.document.height),
            egui::FontId::proportional(18.0),
            self.themes.text_color(),
        );

        // Handle a simple brush stroke with the mouse for the demo.
        // Snapshot the document before the first change of a stroke so undo
        // reverts the whole stroke at once.
        if response.drag_started() {
            self.history.push(self.document.clone());
        }
        if response.dragged() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                // Convert screen pos -> canvas pos using viewport.
                let canvas_pos = self.viewport.screen_to_canvas(pos, rect);
                // Paint a dot on the active raster layer.
                self.document.paint_dot(canvas_pos, 8.0, egui::Color32::BLACK);
            }
        }
    }
}