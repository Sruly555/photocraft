//! Photoshop CS2 (Windows Vista/7) chrome, used when [`crate::theme::is_cs2`] holds.
//!
//! Fork-only. Metrics are measured from lossless CS2 captures; colours are the Vista/7 system
//! colours CS2 drew with. Every drawing helper here is self-contained so the stock code paths only
//! gain a one-line branch.

use egui::{Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

/// Tool button cell (points).
pub const TOOL_CELL: egui::Vec2 = egui::Vec2::new(25.0, 21.0);
/// Gap between tool rows: 21-point cells on a 23-point pitch.
pub const TOOL_ROW_GAP: f32 = 2.0;
/// Toolbox body width: two 25-point columns with 4 points either side.
pub const TOOLBOX_BODY: f32 = 58.0;
/// Inset of the tool grid from the toolbox body edge.
pub const TOOLBOX_INSET: f32 = 4.0;
/// Glyph size inside a tool cell.
pub const TOOL_GLYPH: f32 = 16.0;

/// Vista/7 3D shadow and highlight (COLOR_BTNSHADOW / COLOR_BTNHIGHLIGHT).
pub const SHADOW: Color32 = Color32::from_rgb(160, 160, 160);
pub const HIGHLIGHT: Color32 = Color32::WHITE;
/// Pressed tool button: border and its top-to-bottom fill.
pub const PRESSED_BORDER: Color32 = Color32::from_rgb(112, 112, 112);
pub const PRESSED_TOP: Color32 = Color32::from_rgb(200, 200, 200);
pub const PRESSED_BOTTOM: Color32 = Color32::from_rgb(232, 232, 232);
/// Palette caption strip gradient (inferred from the Vista/7 reference).
pub const CAPTION_TOP: Color32 = Color32::from_rgb(156, 179, 203);
pub const CAPTION_BOTTOM: Color32 = Color32::from_rgb(185, 206, 220);
pub const CAPTION_HEIGHT: f32 = 15.0;

/// Vertical two-stop gradient.
pub fn gradient(painter: &egui::Painter, r: Rect, top: Color32, bottom: Color32) {
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(r.left_top(), top);
    mesh.colored_vertex(r.right_top(), top);
    mesh.colored_vertex(r.right_bottom(), bottom);
    mesh.colored_vertex(r.left_bottom(), bottom);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(mesh);
}

/// A 2-point etched divider (shadow line over highlight line) spanning `width` from the cursor.
pub fn etched_divider(ui: &mut Ui, width: f32) {
    let (r, _) = ui.allocate_exact_size(vec2(width, 2.0), Sense::hover());
    let p = ui.painter();
    p.line_segment([pos2(r.left(), r.top() + 0.5), pos2(r.right(), r.top() + 0.5)], Stroke::new(1.0, SHADOW));
    p.line_segment([pos2(r.left(), r.top() + 1.5), pos2(r.right(), r.top() + 1.5)], Stroke::new(1.0, HIGHLIGHT));
}

/// The pressed (checked) look of a CS2 toolbar button on Vista/7.
pub fn paint_pressed(painter: &egui::Painter, r: Rect) {
    let inner = r.shrink(1.0);
    gradient(painter, inner, PRESSED_TOP, PRESSED_BOTTOM);
    painter.rect_stroke(r, 2.0, Stroke::new(1.0, PRESSED_BORDER), StrokeKind::Inside);
}

/// The hover look: a raised outline (highlight top-left, shadow bottom-right).
pub fn paint_raised(painter: &egui::Painter, r: Rect) {
    let r = r.shrink(0.5);
    painter.line_segment([r.left_bottom(), r.left_top()], Stroke::new(1.0, HIGHLIGHT));
    painter.line_segment([r.left_top(), r.right_top()], Stroke::new(1.0, HIGHLIGHT));
    painter.line_segment([r.right_top(), r.right_bottom()], Stroke::new(1.0, SHADOW));
    painter.line_segment([r.right_bottom(), r.left_bottom()], Stroke::new(1.0, SHADOW));
}

/// A CS2 tool cell: 25 x 21, pressed when selected, raised on hover, icon centred.
pub fn tool_button(ui: &mut Ui, icon: &str, selected: bool, tooltip: &str, show_tip: bool) -> Response {
    let (rect, resp) = ui.allocate_exact_size(TOOL_CELL, Sense::click_and_drag());
    let painter = ui.painter();
    if selected {
        paint_pressed(painter, rect);
    } else if resp.hovered() {
        paint_raised(painter, rect);
    }
    crate::icons::paint(ui, rect, icon, TOOL_GLYPH, Color32::BLACK);
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, ui.is_enabled(), selected, tooltip));
    if tooltip.is_empty() || !show_tip { resp } else { resp.on_hover_text(tooltip) }
}

/// The small solid flyout triangle in a tool cell's bottom-right corner.
pub fn flyout_marker(painter: &egui::Painter, cell: Rect) {
    let b = cell.right_bottom() + vec2(-2.0, -2.0);
    painter.add(egui::Shape::convex_polygon(vec![b, b + vec2(-3.0, 0.0), b + vec2(0.0, -3.0)], Color32::BLACK, Stroke::NONE));
}

/// The light-blue palette caption strip across `r`'s top.
pub fn caption_strip(painter: &egui::Painter, r: Rect) -> Rect {
    let strip = Rect::from_min_size(r.min, vec2(r.width(), CAPTION_HEIGHT));
    gradient(painter, strip, CAPTION_TOP, CAPTION_BOTTOM);
    strip
}

use crate::state::Tool;

/// CS2's toolbox, slot for slot: two columns, four divider-separated sections. Tools CS2 did not
/// have are left out (they stay reachable through their shortcuts and menus).
pub const TOOL_SECTIONS: &[&[&[Tool]]] = &[
    &[
        &[Tool::RectMarquee, Tool::EllipseMarquee, Tool::SingleRowMarquee, Tool::SingleColumnMarquee],
        &[Tool::Move],
        &[Tool::Lasso, Tool::PolygonLasso, Tool::MagneticLasso],
        &[Tool::MagicWand],
        &[Tool::Crop],
        &[Tool::Slice, Tool::SliceSelect],
    ],
    &[
        &[Tool::SpotHealing, Tool::Healing, Tool::Patch, Tool::RedEye],
        &[Tool::Brush, Tool::Pencil, Tool::ColorReplacement],
        &[Tool::CloneStamp, Tool::PatternStamp],
        &[Tool::HistoryBrush, Tool::ArtHistoryBrush],
        &[Tool::Eraser, Tool::BackgroundEraser, Tool::MagicEraser],
        &[Tool::Gradient, Tool::PaintBucket],
        &[Tool::Blur, Tool::Sharpen, Tool::Smudge],
        &[Tool::Dodge, Tool::Burn, Tool::Sponge],
    ],
    &[
        &[Tool::PathSelection, Tool::DirectSelection],
        &[Tool::Type, Tool::VerticalType, Tool::HorizontalTypeMask, Tool::VerticalTypeMask],
        &[Tool::Pen, Tool::FreeformPen, Tool::AddAnchorPoint, Tool::DeleteAnchorPoint, Tool::ConvertPoint],
        &[Tool::Rectangle, Tool::EllipseShape, Tool::Polygon, Tool::Line, Tool::CustomShape],
    ],
    &[&[Tool::Note], &[Tool::Eyedropper, Tool::ColorSampler, Tool::Ruler], &[Tool::Hand], &[Tool::Zoom]],
];
