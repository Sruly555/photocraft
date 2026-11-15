//! CS2's palette well, at the options bar's right end: a store for palettes, as in Photoshop CS2.
//!
//! - Any palette can go in: drag its tab onto the well, or choose Dock to Palette Well from its
//!   palette menu. It leaves the work area, and the well shows its tab.
//! - Clicking a tab in the well drops the palette down under it; a click anywhere else (or on the
//!   tab again) folds it back.
//! - Dragging a tab out of the well floats the palette where it is let go. Showing it from the
//!   Window menu takes it out of the well too.
//!
//! The well holds dock panel ids, and `toolPresets` (the Tool Presets panel, which is not a dock
//! panel). Its contents are saved with the panel layout (`cs2Well`).

use egui::{Color32, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use serde_json::json;

use crate::PhotocraftApp;

/// CS2's default well: Brushes, Tool Presets and Layer Comps.
pub const DEFAULT: &[&str] = &["brushes", "toolPresets", "layerComps"];
/// The Tool Presets panel: kept in the well, but drawn by the preset panels, not the dock.
const TOOL_PRESETS: &str = "toolPresets";
/// Where the well starts in the options bar (measured on CS2 at 1018 and 1920 points wide, it
/// starts 840 points in either way), unless the tool options run further.
pub const WELL_LEFT: f32 = 840.0;

fn open_id() -> egui::Id {
    egui::Id::unique("cs2-well-open")
}
fn geometry_id() -> egui::Id {
    egui::Id::unique("cs2-well-geometry")
}
fn drop_id() -> egui::Id {
    egui::Id::unique("cs2-well-dropdown")
}

/// The well's rect and its tabs' rects, as drawn this frame (for drops and the drop-down).
#[derive(Clone)]
struct Geometry {
    well: Rect,
    tabs: Vec<(String, Rect)>,
}

/// The palettes in the well, in order.
pub fn panels(app: &PhotocraftApp) -> Vec<String> {
    app.ui.cs2_well.clone().unwrap_or_else(|| DEFAULT.iter().map(|s| (*s).to_owned()).collect())
}

fn set_panels(app: &mut PhotocraftApp, panels: Vec<String>) {
    app.ui.cs2_well = Some(panels);
}

/// The well's rect this frame, for a palette tab dragged over it.
pub fn well_rect(ctx: &egui::Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp::<Geometry>(geometry_id())).map(|g| g.well).filter(|r| r.is_positive())
}

/// The label of a palette the well can hold.
fn label(id: &str) -> Option<&'static str> {
    if id == TOOL_PRESETS { Some("Tool Presets") } else { crate::panel_docking::panel_label(id) }
}

/// Put palette `id` in the well (at the end): it closes in the work area.
pub fn dock(app: &mut PhotocraftApp, id: &str) {
    if label(id).is_none() {
        return;
    }
    let mut list = panels(app);
    if !list.iter().any(|p| p == id) {
        list.push(id.to_owned());
    }
    set_panels(app, list);
    if id == TOOL_PRESETS {
        app.ui.presets_ui.tool_presets = false;
    } else if let Err(e) = app.run("window.panel.close", json!({"panel": id})) {
        app.ui.status = e;
    }
}

/// Take palette `id` out of the well and float it with its top left at `at`.
fn undock(app: &mut PhotocraftApp, id: &str, at: egui::Pos2) {
    let list: Vec<String> = panels(app).into_iter().filter(|p| p != id).collect();
    set_panels(app, list);
    if id == TOOL_PRESETS {
        app.ui.presets_ui.tool_presets = true;
        return;
    }
    let size = drop_size(id);
    let params = json!({"panel": id, "x": at.x, "y": at.y, "width": size.x, "height": size.y});
    if let Err(e) = app.run("window.panel.float", params) {
        app.ui.status = e;
    }
}

/// A palette shown in the work area (from the Window menu, say) is no longer in the well.
fn sync(app: &mut PhotocraftApp) {
    let Some(layout) = app.ui.docking.as_ref().map(|w| &w.layout) else { return };
    let list = panels(app);
    let kept: Vec<String> = list.iter().filter(|p| *p == TOOL_PRESETS || !layout.contains(*p)).cloned().collect();
    if kept.len() != list.len() {
        set_panels(app, kept);
    }
    if app.ui.presets_ui.tool_presets && panels(app).iter().any(|p| p == TOOL_PRESETS) {
        let kept: Vec<String> = panels(app).into_iter().filter(|p| p != TOOL_PRESETS).collect();
        set_panels(app, kept);
    }
}

/// The size a palette drops down at, and floats at when dragged out.
fn drop_size(id: &str) -> egui::Vec2 {
    match id {
        "brushes" => vec2(280.0, 430.0),
        "layers" | "channels" | "paths" => vec2(240.0, 380.0),
        "history" | "actions" | "layerComps" => vec2(230.0, 260.0),
        "swatches" | "gradients" | "patterns" => vec2(230.0, 220.0),
        TOOL_PRESETS => vec2(250.0, 320.0),
        _ => vec2(230.0, 240.0),
    }
}

/// The options bar's well group: the separator, the browse button, a separator and the well,
/// filling `ui.max_rect()`. Returns whether the browse button was clicked.
pub fn well(app: &mut PhotocraftApp, ui: &mut egui::Ui, browse_tip: &str) -> bool {
    sync(app);
    let ids: Vec<String> = panels(app).into_iter().filter(|id| label(id).is_some()).collect();
    let labels: Vec<String> = ids.iter().map(|id| tl!(label(id).unwrap_or_default()).to_string()).collect();
    let open = ui.ctx().data(|d| d.get_temp::<String>(open_id()));
    let active = open.as_ref().and_then(|o| ids.iter().position(|id| id == o));
    let labels_ref: Vec<&str> = labels.iter().map(String::as_str).collect();
    let drawn = crate::cs2::palette_well(ui, &labels_ref, active, browse_tip);
    let geometry = Geometry { well: drawn.well, tabs: ids.iter().cloned().zip(drawn.tabs.iter().map(|t| t.rect)).collect() };
    ui.ctx().data_mut(|d| d.insert_temp(geometry_id(), geometry.clone()));
    // A tab dragged out of the well floats its palette where it is let go.
    let count = geometry.tabs.len();
    let mut moved: Option<(usize, usize)> = None;
    for (index, ((id, _), response)) in geometry.tabs.iter().zip(&drawn.tabs).enumerate() {
        let response = &response.response;
        // Right-click: move the tab along the well, as CS2's tab menu did.
        response.context_menu(|ui| {
            let last = count.saturating_sub(1);
            let items = [
                (tl!("Move To Left"), index > 0, index.saturating_sub(1)),
                (tl!("Move To Right"), index < last, (index + 1).min(last)),
                (tl!("Move To Beginning"), index > 0, 0),
                (tl!("Move To End"), index < last, last),
            ];
            for (text, enabled, to) in items {
                if ui.add_enabled(enabled, egui::Button::new(text)).clicked() {
                    moved = Some((index, to));
                    ui.close();
                }
            }
        });
        if response.dragged()
            && let Some(p) = ui.ctx().pointer_interact_pos()
            && !drawn.well.expand(6.0).contains(p)
        {
            // The palette's outline under the pointer, as CS2 drew it while dragging.
            let ghost = Rect::from_min_size(p - vec2(20.0, 8.0), drop_size(id));
            let painter = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::unique("cs2-well-ghost")));
            painter.rect_stroke(ghost, 0.0, Stroke::new(2.0, Color32::from_gray(60)), StrokeKind::Inside);
        }
        if response.drag_stopped()
            && let Some(p) = ui.ctx().pointer_interact_pos().or_else(|| ui.input(|i| i.pointer.latest_pos()))
            && !drawn.well.expand(6.0).contains(p)
        {
            ui.ctx().data_mut(|d| d.remove::<String>(open_id()));
            undock(app, id, p - vec2(20.0, 8.0));
        } else if response.clicked() {
            let again = open.as_deref() == Some(id.as_str());
            ui.ctx().data_mut(|d| {
                if again {
                    d.remove::<String>(open_id());
                } else {
                    d.insert_temp(open_id(), id.clone());
                }
            });
        }
    }
    if let Some((from, to)) = moved {
        let ids: Vec<String> = geometry.tabs.iter().map(|(id, _)| id.clone()).collect();
        set_panels(app, reorder(panels(app), &ids, from, to));
    }
    drawn.browse
}

/// `list` with the shown tab at `from` moved to shown position `to`. `shown` are the tabs the
/// well draws (the list's palettes it can show), so a stored id the build doesn't know keeps its
/// place.
fn reorder(mut list: Vec<String>, shown: &[String], from: usize, to: usize) -> Vec<String> {
    let (Some(id), Some(target)) = (shown.get(from), shown.get(to)) else { return list };
    if from == to {
        return list;
    }
    let Some(at) = list.iter().position(|p| p == id) else { return list };
    let moving = list.remove(at);
    let Some(mut dest) = list.iter().position(|p| p == target) else {
        list.insert(at.min(list.len()), moving);
        return list;
    };
    if to > from {
        dest += 1;
    }
    list.insert(dest.min(list.len()), moving);
    list
}

/// The palette dropped down from the well, if one is: under its tab, over everything else. A
/// press anywhere but on it, a popup of its own or the well's tabs folds it back.
pub fn dropdown(app: &mut PhotocraftApp, ctx: &egui::Context) {
    let Some(open) = ctx.data(|d| d.get_temp::<String>(open_id())) else { return };
    let Some(geometry) = ctx.data(|d| d.get_temp::<Geometry>(geometry_id())) else { return };
    let Some(tab) = geometry.tabs.iter().find(|(id, _)| *id == open).map(|(_, r)| *r) else {
        ctx.data_mut(|d| d.remove::<String>(open_id()));
        return;
    };
    let screen = ctx.content_rect();
    let size = drop_size(&open).min(screen.size() - vec2(8.0, geometry.well.bottom() + 8.0)).max(vec2(160.0, 120.0));
    let x = tab.left().min(screen.right() - size.x - 2.0).max(screen.left() + 2.0);
    let rect = Rect::from_min_size(pos2(x, geometry.well.bottom() - 1.0), size);
    let t = crate::theme::Tokens::get(ctx);
    let shown = egui::Area::new(drop_id()).fixed_pos(rect.min).order(egui::Order::Foreground).show(ctx, |ui| {
        let (frame, _) = ui.allocate_exact_size(rect.size(), Sense::click());
        ui.painter().rect_filled(frame, 0.0, t.chrome);
        let body = frame.shrink2(vec2(6.0, 5.0));
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).id_salt(("cs2-well-body", open.as_str())));
        child.set_clip_rect(body);
        if open == TOOL_PRESETS {
            crate::preset_panels::tool_presets_panel(app, &mut child);
        } else {
            crate::panel_docking::panel_body(app, &mut child, &open);
        }
        ui.painter().rect_stroke(frame, 0.0, Stroke::new(1.0, crate::cs2::FRAME), StrokeKind::Inside);
    });
    let layer = shown.response.layer_id;
    // Read the press first, then ask which layer is under it: `layer_id_at` takes the context's
    // lock, and taking it inside `ctx.input` deadlocks the window when another thread waits on it.
    let pressed = ctx.input(|i| if i.pointer.any_pressed() { i.pointer.interact_pos() } else { None });
    let fold = pressed.is_some_and(|p| {
        let on_tabs = geometry.tabs.iter().any(|(_, r)| r.contains(p));
        // A popup the palette opened (a menu, a combo box) is on another foreground layer.
        let on_popup = ctx.layer_id_at(p).is_some_and(|l| l != layer && matches!(l.order, egui::Order::Foreground | egui::Order::Tooltip));
        !rect.contains(p) && !on_tabs && !on_popup
    });
    if fold || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        ctx.data_mut(|d| d.remove::<String>(open_id()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> PhotocraftApp {
        PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default())
    }

    #[test]
    fn the_well_starts_with_cs2_palettes_and_takes_any_palette() {
        let mut app = app();
        assert_eq!(panels(&app), ["brushes", "toolPresets", "layerComps"]);
        dock(&mut app, "navigator");
        assert_eq!(panels(&app), ["brushes", "toolPresets", "layerComps", "navigator"]);
        // Twice is once; an unknown id is refused.
        dock(&mut app, "navigator");
        dock(&mut app, "no such palette");
        assert_eq!(panels(&app).len(), 4);
        undock(&mut app, "brushes", pos2(400.0, 200.0));
        assert_eq!(panels(&app), ["toolPresets", "layerComps", "navigator"]);
    }

    #[test]
    fn the_tab_menu_moves_a_tab_along_the_well() {
        let ids = |v: &[&str]| v.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        let shown = ids(&["a", "b", "c", "d"]);
        assert_eq!(reorder(shown.clone(), &shown, 2, 1), ids(&["a", "c", "b", "d"]));
        assert_eq!(reorder(shown.clone(), &shown, 1, 2), ids(&["a", "c", "b", "d"]));
        assert_eq!(reorder(shown.clone(), &shown, 2, 0), ids(&["c", "a", "b", "d"]));
        assert_eq!(reorder(shown.clone(), &shown, 0, 3), ids(&["b", "c", "d", "a"]));
        // Out of range or in place: unchanged.
        assert_eq!(reorder(shown.clone(), &shown, 9, 0), shown);
        assert_eq!(reorder(shown.clone(), &shown, 1, 1), shown);
        // An id the well can't show keeps its place in the stored list.
        let stored = ids(&["a", "x", "b", "c"]);
        assert_eq!(reorder(stored, &ids(&["a", "b", "c"]), 2, 0), ids(&["c", "a", "x", "b"]));
    }

    #[test]
    fn the_well_is_saved_with_the_panel_layout() {
        let mut app = app();
        dock(&mut app, "info");
        let saved = crate::dock::snapshot(&app);
        let mut next = self::app();
        crate::dock::apply(&mut next, &saved);
        assert_eq!(panels(&next), panels(&app));
    }
}
