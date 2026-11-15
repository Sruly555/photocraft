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

// ------------------------------------------------------------------------------- palette groups

/// Palette caption strip (Vista/7 small caption, inferred) above the 16-point tab row (measured).
pub const PALETTE_CAPTION: f32 = CAPTION_HEIGHT;
pub const PALETTE_TAB_ROW: f32 = 16.0;
/// The whole group strip: caption + tab row.
pub const PALETTE_STRIP: f32 = PALETTE_CAPTION + PALETTE_TAB_ROW;
/// A tab's width beyond its laid-out label: 4 margin + 6 inset before the text, the rest after
/// it, so the next label's ink starts 17 points after this one's (measured; a galley is ~2.5
/// points wider than its ink at this size).
pub const PALETTE_TAB_PADDING: f32 = 14.5;
/// Tab label size: Tahoma 8 pt in CS2; PhotoCraft's UI face matches its widths at 9.5.
pub const PALETTE_FONT: f32 = 9.5;
const TAB_MARGIN: f32 = 4.0;
const TAB_INSET: f32 = 6.0;
const SLOPE_GAP: f32 = 5.0;
const SLOPE_WIDTH: f32 = 17.0;
/// Width of the group header at the strip's right: caption buttons over the round menu button.
pub const PALETTE_HEADER: f32 = 34.0;
/// The tab row behind the tabs: flat grey over a white baseline (measured, neutral in both themes).
const ROW_GREY: Color32 = Color32::from_rgb(178, 178, 178);
const POCKET_SHADOW: Color32 = Color32::from_rgb(151, 151, 151);
/// The light pocket for the round menu button: a ramp rising from the baseline 30 points from the
/// right edge to the 4th row 14 points from it (measured).
const POCKET_BOTTOM: f32 = 30.0;
const POCKET_TOP: f32 = 14.0;
const POCKET_ROW: f32 = 4.0;
const INACTIVE_TOP: Color32 = Color32::from_rgb(208, 208, 208);
const INACTIVE_BOTTOM: Color32 = Color32::from_rgb(169, 169, 169);
const TAB_OUTLINE: Color32 = Color32::from_rgb(122, 122, 122);
pub const INACTIVE_TEXT: Color32 = Color32::from_rgb(72, 72, 72);

fn tab_row(strip: Rect) -> Rect {
    Rect::from_min_max(pos2(strip.left(), strip.bottom() - PALETTE_TAB_ROW), strip.max)
}

/// The caption strip and the tab row's background (flat grey, white baseline, the menu button's
/// pocket at the right); the tabs then cover it.
pub fn paint_palette_strip(painter: &egui::Painter, strip: Rect, body: Color32) {
    caption_strip(painter, strip);
    let row = tab_row(strip);
    painter.rect_filled(Rect::from_min_max(row.min, pos2(row.right(), row.bottom() - 1.0)), 0.0, ROW_GREY);
    let base = row.bottom() - 0.5;
    painter.line_segment([pos2(row.left(), base), pos2(row.right(), base)], Stroke::new(1.0, Color32::WHITE));
    // The pocket: a mirrored slope from the baseline up to row 4, light to the right edge.
    let (r, top, bottom) = (row.right(), row.top() + POCKET_ROW, row.bottom());
    let steps = 12;
    let mut ramp = Vec::new();
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = r - POCKET_BOTTOM + (POCKET_BOTTOM - POCKET_TOP) * t.powf(0.85);
        ramp.push(pos2(x, bottom - (bottom - top) * t));
    }
    let mut pocket = ramp.clone();
    pocket.push(pos2(r, top));
    pocket.push(pos2(r, bottom));
    painter.add(egui::Shape::convex_polygon(pocket, body, Stroke::NONE));
    let mut edge = ramp;
    edge.push(pos2(r, top));
    painter.add(egui::Shape::line(edge.iter().map(|p| *p + vec2(-0.5, -0.5)).collect(), Stroke::new(1.0, POCKET_SHADOW)));
}

/// The slope's x at fraction `t` (0 = top, 1 = bottom): near-straight, softened at the top.
fn slope_x(start: f32, t: f32) -> f32 {
    start + SLOPE_WIDTH * t.clamp(0.0, 1.0).powf(0.85)
}

/// A tab's outline: left edge, flat top at `top`, the slope down to `bottom`.
fn tab_outline(left: f32, slope_start: f32, top: f32, bottom: f32) -> Vec<egui::Pos2> {
    let mut pts = vec![pos2(left, bottom), pos2(left, top)];
    let steps = 12;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        pts.push(pos2(slope_x(slope_start, t), top + (bottom - top) * t));
    }
    pts
}

/// Fill a convex outline with a vertical gradient (per-vertex colour by height).
fn fill_vertical(painter: &egui::Painter, pts: &[egui::Pos2], top: f32, bottom: f32, c0: Color32, c1: Color32) {
    let mut mesh = egui::Mesh::default();
    let span = (bottom - top).max(1.0);
    for p in pts {
        let t = ((p.y - top) / span).clamp(0.0, 1.0);
        mesh.colored_vertex(*p, lerp(c0, c1, t));
    }
    for i in 1..pts.len().saturating_sub(1) {
        mesh.add_triangle(0, i as u32, i as u32 + 1);
    }
    painter.add(mesh);
}

fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let m = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

/// One tab to paint: its hit rect, laid-out label, and whether it is the active one.
pub struct PaletteTab {
    pub rect: Rect,
    pub galley: std::sync::Arc<egui::Galley>,
    pub active: bool,
    /// Room after the label, before the slope (a document tab's close button).
    pub extra: f32,
}

/// CS2's tabs: inactive ones right to left (each slope over its right neighbour), the active one
/// last, on top. Shapes overlap neighbours; only the painting does, hit areas stay the stock ones.
pub fn paint_palette_tabs(painter: &egui::Painter, strip: Rect, body: Color32, tabs: &[PaletteTab]) {
    let row = tab_row(strip);
    let painter = painter.with_clip_rect(row.intersect(painter.clip_rect()));
    let shape = |tab: &PaletteTab| {
        let left = tab.rect.left() + TAB_MARGIN;
        let text_end = left + TAB_INSET + tab.galley.size().x + tab.extra;
        (left, text_end + SLOPE_GAP)
    };
    let text_at = |tab: &PaletteTab, left: f32| pos2(left + TAB_INSET, row.top() + 2.0 + (PALETTE_TAB_ROW - 2.0 - tab.galley.size().y) / 2.0);
    for tab in tabs.iter().rev().filter(|t| !t.active) {
        let (left, s) = shape(tab);
        let (top, bottom) = (row.top() + 2.0, row.bottom() - 1.0);
        let pts = tab_outline(left, s, top, bottom);
        fill_vertical(&painter, &pts, top, bottom, INACTIVE_TOP, INACTIVE_BOTTOM);
        let slope: Vec<_> = pts[2..].to_vec();
        painter.add(egui::Shape::line(slope, Stroke::new(1.0, TAB_OUTLINE)));
        painter.line_segment([pos2(left, row.bottom() - 0.5), pos2(slope_x(s, 1.0) + 1.0, row.bottom() - 0.5)], Stroke::new(1.0, Color32::WHITE));
        painter.galley(text_at(tab, left), tab.galley.clone(), INACTIVE_TEXT);
    }
    for tab in tabs.iter().filter(|t| t.active) {
        let (left, s) = shape(tab);
        let (top, bottom) = (row.top() + 1.0, row.bottom());
        let pts = tab_outline(left, s, top, bottom);
        painter.add(egui::Shape::convex_polygon(pts.clone(), body, Stroke::NONE));
        // Highlights: top, left edge, and along the slope.
        painter.line_segment([pos2(left, top + 0.5), pos2(s, top + 0.5)], Stroke::new(1.0, Color32::WHITE));
        painter.line_segment([pos2(left + 0.5, top), pos2(left + 0.5, bottom)], Stroke::new(1.0, Color32::WHITE));
        let edge: Vec<_> = pts[2..].iter().map(|p| *p + vec2(0.5, 0.0)).collect();
        painter.add(egui::Shape::line(edge, Stroke::new(1.0, Color32::from_rgb(250, 250, 250))));
        painter.galley(text_at(tab, left), tab.galley.clone(), Color32::BLACK);
    }
}

/// A Vista/7 caption button: 13 x 11, rounded, gradient face.
fn caption_button(painter: &egui::Painter, r: Rect, close: bool, hovered: bool) {
    let (top, bottom, border) = if close {
        (Color32::from_rgb(232, 140, 130), Color32::from_rgb(199, 80, 80), Color32::from_rgb(130, 55, 50))
    } else {
        (Color32::WHITE, Color32::from_rgb(214, 222, 232), Color32::from_rgb(120, 130, 145))
    };
    let (top, bottom) = if hovered { (lerp(top, Color32::WHITE, 0.3), lerp(bottom, Color32::WHITE, 0.3)) } else { (top, bottom) };
    gradient(painter, r.shrink(1.0), top, bottom);
    painter.rect_stroke(r, 2.0, Stroke::new(1.0, border), StrokeKind::Inside);
    let ink = if close { Color32::WHITE } else { Color32::BLACK };
    let c = r.center();
    if close {
        let s = Stroke::new(1.5, ink);
        painter.line_segment([c + vec2(-2.5, -2.5), c + vec2(2.5, 2.5)], s);
        painter.line_segment([c + vec2(2.5, -2.5), c + vec2(-2.5, 2.5)], s);
    } else {
        painter.rect_filled(Rect::from_center_size(c + vec2(0.0, 2.0), vec2(6.0, 2.0)), 0.0, ink);
    }
}

/// What the header's buttons asked for this frame.
#[derive(Default)]
pub struct HeaderClicks {
    pub minimize: bool,
    pub close: bool,
    pub menu: Option<Response>,
}

/// The group header at the strip's right: minimise and close in the caption, the round ▶ menu
/// button in the tab row.
pub fn palette_header(ui: &mut Ui, minimize_tip: &str, close_tip: &str, menu_tip: &str) -> HeaderClicks {
    let area = ui.max_rect();
    let cap_y = area.top() + PALETTE_CAPTION / 2.0;
    let close_r = Rect::from_center_size(pos2(area.right() - 9.0, cap_y), vec2(13.0, 11.0));
    let min_r = close_r.translate(vec2(-15.0, 0.0));
    let mut out = HeaderClicks::default();
    let m = ui.interact(min_r, ui.id().with("cs2-min"), Sense::click()).on_hover_text(minimize_tip);
    caption_button(ui.painter(), min_r, false, m.hovered());
    out.minimize = m.clicked();
    let c = ui.interact(close_r, ui.id().with("cs2-close"), Sense::click()).on_hover_text(close_tip);
    caption_button(ui.painter(), close_r, true, c.hovered());
    out.close = c.clicked();
    // Round menu button: bevelled disc, black triangle.
    let row_y = area.bottom() - PALETTE_TAB_ROW / 2.0;
    let centre = pos2(area.right() - 9.0, row_y);
    let br = Rect::from_center_size(centre, vec2(14.0, 14.0));
    let b = ui.interact(br, ui.id().with("cs2-menu"), Sense::click()).on_hover_text(menu_tip);
    let p = ui.painter();
    p.circle_filled(centre + vec2(0.5, 0.5), 6.5, SHADOW);
    p.circle_filled(centre, 6.5, if b.hovered() { Color32::WHITE } else { Color32::from_rgb(244, 244, 244) });
    p.circle_stroke(centre, 6.5, Stroke::new(1.0, Color32::from_rgb(138, 138, 138)));
    p.add(egui::Shape::convex_polygon(
        vec![centre + vec2(-1.5, -3.0), centre + vec2(2.5, 0.0), centre + vec2(-1.5, 3.0)],
        Color32::BLACK,
        Stroke::NONE,
    ));
    out.menu = Some(b);
    out
}

// ------------------------------------------------------------------------------ document tabs

/// Document tab strip: a CS2 tab row over a 2-point body-coloured band the active tab joins.
pub const DOC_BAND: f32 = 2.0;
pub const DOC_STRIP: f32 = PALETTE_TAB_ROW + DOC_BAND;
/// Room for a document tab's close button after its label.
pub const DOC_CLOSE_ROOM: f32 = 12.0;
pub const DOC_TAB_PADDING: f32 = PALETTE_TAB_PADDING + DOC_CLOSE_ROOM;

/// The document strip's background; returns the tab row (its top 16 points).
pub fn paint_doc_strip(painter: &egui::Painter, strip: Rect, body: Color32) -> Rect {
    let row = Rect::from_min_size(strip.min, vec2(strip.width(), PALETTE_TAB_ROW));
    painter.rect_filled(Rect::from_min_max(row.min, pos2(row.right(), row.bottom() - 1.0)), 0.0, ROW_GREY);
    let base = row.bottom() - 0.5;
    painter.line_segment([pos2(row.left(), base), pos2(row.right(), base)], Stroke::new(1.0, Color32::WHITE));
    painter.rect_filled(Rect::from_min_max(pos2(strip.left(), row.bottom()), strip.max), 0.0, body);
    row
}

/// Where a document tab's label starts and its close button sits, in the tab row.
pub fn doc_tab_layout(tab: Rect, row: Rect, label: egui::Vec2) -> (egui::Pos2, Rect) {
    let left = tab.left() + TAB_MARGIN + TAB_INSET;
    let text = pos2(left, row.top() + 2.0 + (PALETTE_TAB_ROW - 2.0 - label.y) / 2.0);
    let close = Rect::from_center_size(pos2(left + label.x + 3.0 + 4.5, row.center().y + 1.0), vec2(9.0, 9.0));
    (text, close)
}

/// A document tab's close button: a small × (raised box on hover).
pub fn paint_doc_close(painter: &egui::Painter, r: Rect, active: bool, hovered: bool) {
    if hovered {
        paint_raised(painter, r);
    }
    let ink = if active || hovered { Color32::BLACK } else { INACTIVE_TEXT };
    let c = r.center();
    let s = Stroke::new(1.2, ink);
    painter.line_segment([c + vec2(-2.5, -2.5), c + vec2(2.5, 2.5)], s);
    painter.line_segment([c + vec2(2.5, -2.5), c + vec2(-2.5, 2.5)], s);
}
