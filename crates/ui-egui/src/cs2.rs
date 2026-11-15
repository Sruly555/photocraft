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
/// Tab label size: Tahoma 8 pt in CS2; PhotoCraft's UI face matches its widths at 9.5 points
/// on screen, which is 11.4 before the Tiny UI scale CS2 runs at.
pub const PALETTE_FONT: f32 = 11.4;
const TAB_MARGIN: f32 = 4.0;
const TAB_INSET: f32 = 6.0;
/// The slope starts 4 points after the label's ink: 1.5 after its laid-out width (measured).
const SLOPE_GAP: f32 = 1.5;
const SLOPE_WIDTH: f32 = 17.0;
/// Width of the group header at the strip's right: caption buttons over the round menu button.
pub const PALETTE_HEADER: f32 = 34.0;
/// The tab row behind the tabs: flat grey over a white baseline (measured, neutral in both themes).
const ROW_GREY: Color32 = Color32::from_rgb(178, 178, 178);
const POCKET_SHADOW: Color32 = Color32::from_rgb(151, 151, 151);
/// The light pocket for the round menu button: a ramp rising from the baseline 30 points from the
/// right edge to the 4th row 14 points from it (measured).
const POCKET_BOTTOM: f32 = 29.5;
const POCKET_TOP: f32 = 16.5;
/// The soft shadow the shelf casts on the grey row, left of its slope.
const POCKET_SHADE: f32 = 7.0;
const POCKET_ROW: f32 = 4.0;
const INACTIVE_TOP: Color32 = Color32::from_rgb(208, 208, 208);
const INACTIVE_BOTTOM: Color32 = Color32::from_rgb(169, 169, 169);
const TAB_OUTLINE: Color32 = Color32::from_rgb(122, 122, 122);
pub const INACTIVE_TEXT: Color32 = Color32::from_rgb(72, 72, 72);
/// A following tab's left edge: dark line, then a lighter one (measured).
const EDGE_DARK: Color32 = Color32::from_rgb(95, 95, 95);
const EDGE_LIGHT: Color32 = Color32::from_rgb(165, 165, 165);

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
    // The shade: darkest against the slope, fading into the row over seven points.
    let mut shade = egui::Mesh::default();
    for p in &ramp {
        shade.colored_vertex(*p + vec2(-POCKET_SHADE, 0.0), ROW_GREY);
        shade.colored_vertex(*p, POCKET_SHADOW);
    }
    for i in 0..steps as u32 {
        let k = i * 2;
        shade.add_triangle(k, k + 1, k + 2);
        shade.add_triangle(k + 1, k + 2, k + 3);
    }
    painter.add(shade);
    // A one-point shadow line above the shelf's flat top.
    painter.rect_filled(Rect::from_min_max(pos2(r - POCKET_TOP - 4.0, top - 1.0), pos2(r, top)), 0.0, POCKET_SHADOW);
    let mut pocket = ramp.clone();
    pocket.push(pos2(r, top));
    pocket.push(pos2(r, bottom));
    painter.add(egui::Shape::convex_polygon(pocket, body, Stroke::NONE));
    // White highlight just inside the slope and along the top.
    let mut edge: Vec<_> = ramp.iter().map(|p| *p + vec2(1.0, 0.0)).collect();
    edge.push(pos2(r, top + 0.5));
    if let Some(last) = edge.get_mut(steps) {
        last.y = top + 0.5;
    }
    painter.add(egui::Shape::line(edge, Stroke::new(1.0, Color32::WHITE)));
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
/// `edge_all`: every tab has its dark left edge (the palette well's tabs), not only those after
/// the first.
pub fn paint_palette_tabs(painter: &egui::Painter, strip: Rect, body: Color32, tabs: &[PaletteTab], edge_all: bool) {
    let row = tab_row(strip);
    let painter = painter.with_clip_rect(row.intersect(painter.clip_rect()));
    // The slope starts 5 points after the label, or at the tab's own right edge when the row is
    // squeezed (the next tab's edge then cuts the label, as in CS2's palette well).
    let shape = |tab: &PaletteTab| {
        let left = tab.rect.left() + TAB_MARGIN;
        let text_end = left + TAB_INSET + tab.galley.size().x + tab.extra;
        (left, (text_end + SLOPE_GAP).min(tab.rect.right() + 0.5))
    };
    let first = tabs.first().map(|t| t.rect.left());
    let text_at = |tab: &PaletteTab, left: f32| pos2(left + TAB_INSET, row.top() + 2.0 + (PALETTE_TAB_ROW - 2.0 - tab.galley.size().y) / 2.0);
    // Palettes: right to left, each slope over its right neighbour. The palette well (`edge_all`):
    // left to right, each tab's edge cutting the label before it (measured, both).
    let order: Vec<&PaletteTab> = if edge_all { tabs.iter().collect() } else { tabs.iter().rev().collect() };
    for tab in order.into_iter().filter(|t| !t.active) {
        let (left, s) = shape(tab);
        let (top, bottom) = (row.top() + 2.0, row.bottom() - 1.0);
        // A tab after the first starts at a dark edge 6 points before its label; the tab to its
        // left (painted later, on top) covers that edge's lower part with its slope (measured).
        let after_first = edge_all || first.is_some_and(|f| tab.rect.left() > f + 0.5);
        let edge = left;
        let pts = tab_outline(edge, s.max(edge + 1.0), top, bottom);
        fill_vertical(&painter, &pts, top, bottom, INACTIVE_TOP, INACTIVE_BOTTOM);
        if after_first {
            painter.line_segment([pos2(edge + 0.5, top), pos2(edge + 0.5, bottom)], Stroke::new(1.0, EDGE_DARK));
            painter.line_segment([pos2(edge + 1.5, top), pos2(edge + 1.5, bottom)], Stroke::new(1.0, EDGE_LIGHT));
        }
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
    caption_button_glyph(painter, r, if close { Caption::Close } else { Caption::Minimize }, hovered);
}

/// What a caption button shows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Caption {
    Minimize,
    Maximize,
    Close,
}

/// A caption button of `kind` (see [`caption_button`]).
pub fn caption_button_glyph(painter: &egui::Painter, r: Rect, kind: Caption, hovered: bool) {
    let close = kind == Caption::Close;
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
    } else if kind == Caption::Maximize {
        let b = Rect::from_center_size(c, vec2(7.0, 6.0));
        painter.rect_stroke(b, 0.0, Stroke::new(1.0, ink), StrokeKind::Inside);
        painter.rect_filled(Rect::from_min_size(b.min, vec2(b.width(), 2.0)), 0.0, ink);
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
    let m = ui.interact(min_r, ui.scope_id().with("cs2-min"), Sense::click()).on_hover_text(minimize_tip);
    caption_button(ui.painter(), min_r, false, m.hovered());
    out.minimize = m.clicked();
    let c = ui.interact(close_r, ui.scope_id().with("cs2-close"), Sense::click()).on_hover_text(close_tip);
    caption_button(ui.painter(), close_r, true, c.hovered());
    out.close = c.clicked();
    // Round menu button on the shelf: an 11-point disc, lit from the top left, its ring darker
    // to the bottom right, with a small black arrow.
    let centre = pos2(area.right() - 6.5, area.bottom() - 5.5);
    let br = Rect::from_center_size(centre, vec2(13.0, 13.0));
    let b = ui.interact(br, ui.scope_id().with("cs2-menu"), Sense::click()).on_hover_text(menu_tip);
    menu_disc(ui.painter(), centre, b.hovered(), b.is_pointer_button_down_on());
    out.menu = Some(b);
    out
}

/// The palette menu button's disc at `centre`.
fn menu_disc(p: &egui::Painter, centre: egui::Pos2, hovered: bool, pressed: bool) {
    const R: f32 = 5.5;
    const SEGS: usize = 24;
    let (lit, dim) = match (pressed, hovered) {
        (true, _) => (Color32::from_gray(200), Color32::from_gray(232)),
        (false, true) => (Color32::WHITE, Color32::from_gray(228)),
        (false, false) => (Color32::WHITE, Color32::from_gray(212)),
    };
    // Fill: a fan whose vertex colours run from top left (lit) to bottom right (dim).
    let shade = |v: egui::Vec2| ((v.x + v.y) / (2.0 * R) * 0.5 + 0.5).clamp(0.0, 1.0);
    let mut fill = egui::Mesh::default();
    fill.colored_vertex(centre, lerp(lit, dim, 0.5));
    for i in 0..=SEGS {
        let a = i as f32 / SEGS as f32 * std::f32::consts::TAU;
        let v = vec2(a.cos(), a.sin()) * R;
        fill.colored_vertex(centre + v, lerp(lit, dim, shade(v)));
    }
    for i in 1..=SEGS as u32 {
        fill.add_triangle(0, i, i + 1);
    }
    p.add(fill);
    // Ring: about 154 at the top left, 125 at the bottom right.
    for i in 0..SEGS {
        let a0 = i as f32 / SEGS as f32 * std::f32::consts::TAU;
        let a1 = (i + 1) as f32 / SEGS as f32 * std::f32::consts::TAU;
        let v0 = vec2(a0.cos(), a0.sin()) * (R - 0.5);
        let v1 = vec2(a1.cos(), a1.sin()) * (R - 0.5);
        let c = lerp(Color32::from_gray(154), Color32::from_gray(125), shade(v0 + v1));
        p.line_segment([centre + v0, centre + v1], Stroke::new(1.0, c));
    }
    p.add(egui::Shape::convex_polygon(vec![centre + vec2(-2.5, -2.5), centre + vec2(0.75, 0.0), centre + vec2(-2.5, 2.5)], Color32::BLACK, Stroke::NONE));
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
    // The empty part of the row is the window's light grey; only the tabs themselves are drawn.
    painter.rect_filled(Rect::from_min_max(row.min, pos2(row.right(), row.bottom() - 1.0)), 0.0, body);
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

// --------------------------------------------------------------------------- bars and separators

/// Options bar height (measured: 31 points under a 19-point menu bar and its etched line).
pub const OPTIONS_BAR: f32 = 31.0;

/// A vertical etched separator (shadow over highlight) centred in `r`.
pub fn etched_vline(painter: &egui::Painter, r: Rect) {
    let x = r.center().x.floor();
    painter.line_segment([pos2(x + 0.5, r.top()), pos2(x + 0.5, r.bottom())], Stroke::new(1.0, SHADOW));
    painter.line_segment([pos2(x + 1.5, r.top()), pos2(x + 1.5, r.bottom())], Stroke::new(1.0, HIGHLIGHT));
}

/// The dotted drag grip at a bar's start: a column of 1-point dots, shadow with a highlight
/// below-right. Built as a raw mesh: egui's anti-aliasing drops 1-point rects and lines.
pub fn grip(ui: &mut Ui, height: f32) {
    let (r, _) = ui.allocate_exact_size(vec2(8.0, height), Sense::hover());
    let x = r.left().round() + 2.0;
    let mut y = r.top().round() + 1.0;
    let mut mesh = egui::Mesh::default();
    while y + 2.0 <= r.bottom() {
        pixel(&mut mesh, pos2(x + 1.0, y + 1.0), HIGHLIGHT);
        pixel(&mut mesh, pos2(x, y), SHADOW);
        y += 2.0;
    }
    ui.painter().add(mesh);
}

/// One crisp pixel-sized quad at `at` (top-left) in a mesh.
pub fn pixel(mesh: &mut egui::Mesh, at: egui::Pos2, color: Color32) {
    mesh.add_colored_rect(Rect::from_min_size(at, vec2(1.0, 1.0)), color);
}

/// A floating palette's frame (the toolbox's and the palette windows').
pub const FRAME: Color32 = Color32::from_rgb(122, 122, 122);
/// The floating toolbox's height: caption and header, CS2's four tool sections (measured 71, 93,
/// 49 and 49 points) and its footer.
pub const TOOLBOX_HEIGHT: f32 = CAPTION_HEIGHT + 23.0 + 71.0 + 93.0 + 49.0 + 49.0 + 2.0 + 53.0 + 2.0 + 27.0 + 2.0 + 22.0 + 2.0 + 25.0 + 4.0;

// -------------------------------------------------------------------------------- palette well

/// The palette well at the options bar's right end (measured): 166 x 23 points, sunken, grey,
/// with palette tabs standing on its bottom edge.
pub const WELL_SIZE: egui::Vec2 = egui::Vec2::new(166.0, 23.0);
const WELL_GREY: Color32 = Color32::from_rgb(178, 178, 178);
/// The options-bar room for the separator, the Go to Bridge button, a separator and the well at
/// its narrowest.
pub const WELL_AREA: f32 = 9.0 + 26.0 + 9.0 + WELL_SIZE.x + 6.0;

/// A small folder under a magnifier: CS2's "Go to Bridge" button position, drawn original.
fn browse_glyph(p: &egui::Painter, c: egui::Pos2) {
    let body = Rect::from_center_size(c + vec2(-2.0, 2.0), vec2(14.0, 10.0));
    let tab = Rect::from_min_size(body.min + vec2(0.0, -2.0), vec2(6.0, 3.0));
    p.rect_filled(tab, 0.0, Color32::from_gray(150));
    p.rect_filled(body, 1.0, Color32::from_gray(230));
    p.rect_stroke(body, 1.0, Stroke::new(1.0, Color32::from_gray(80)), StrokeKind::Inside);
    let lens = c + vec2(4.0, -3.0);
    p.circle_filled(lens, 3.5, Color32::WHITE);
    p.circle_stroke(lens, 3.5, Stroke::new(1.2, Color32::from_gray(40)));
    p.line_segment([lens + vec2(2.5, 2.5), lens + vec2(5.5, 5.5)], Stroke::new(2.0, Color32::from_gray(40)));
}

/// A tab of the palette well, as drawn this frame.
pub struct WellTab {
    pub rect: Rect,
    pub response: Response,
}

/// What [`palette_well`] drew.
pub struct WellDrawn {
    /// The sunken well.
    pub well: Rect,
    pub tabs: Vec<WellTab>,
    /// The browse button was clicked.
    pub browse: bool,
}

/// Draws the well's group into `ui.max_rect()`: a separator, the browse button, a separator and
/// the well, which takes the rest of the width (CS2's well runs to the options bar's end). The
/// tabs stand on the well's floor, `active` (a palette dropped down from it) drawn in front.
pub fn palette_well(ui: &mut Ui, labels: &[&str], active: Option<usize>, browse_tip: &str) -> WellDrawn {
    let area = ui.max_rect();
    let p = ui.painter().clone();
    let cy = area.center().y;
    let mut x = area.left();
    let sep = |x: f32| etched_vline(&p, Rect::from_min_max(pos2(x, cy - 11.0), pos2(x + 9.0, cy + 11.0)));
    sep(x);
    x += 9.0;
    let browse = Rect::from_min_size(pos2(x, cy - 12.0), vec2(26.0, 24.0));
    let b = ui.interact(browse, ui.scope_id().with("cs2-browse"), Sense::click()).on_hover_text(browse_tip);
    if b.hovered() {
        paint_raised(&p, browse);
    }
    browse_glyph(&p, browse.center());
    x += 26.0;
    sep(x);
    x += 9.0;
    // The sunken well: shadow top and left, highlight bottom and right.
    let width = (area.right() - 4.0 - x).max(WELL_SIZE.x);
    let well = Rect::from_min_size(pos2(x, (cy - WELL_SIZE.y / 2.0).round()), vec2(width, WELL_SIZE.y));
    p.rect_filled(well, 0.0, WELL_GREY);
    p.line_segment([well.left_top() + vec2(0.5, 0.5), well.right_top() + vec2(-0.5, 0.5)], Stroke::new(1.0, SHADOW));
    p.line_segment([well.left_top() + vec2(0.5, 0.5), well.left_bottom() + vec2(0.5, -0.5)], Stroke::new(1.0, SHADOW));
    p.line_segment([well.right_top() + vec2(-0.5, 0.5), well.right_bottom() + vec2(-0.5, -0.5)], Stroke::new(1.0, HIGHLIGHT));
    // Tabs on the well's floor at their natural widths; when they don't fit, the earlier tabs
    // give up room (CS2 cut their labels without an ellipsis).
    let row = Rect::from_min_max(pos2(well.left() - 1.0, well.bottom() - PALETTE_TAB_ROW), pos2(well.right() - 1.0, well.bottom()));
    let font = egui::FontId::proportional(PALETTE_FONT);
    let natural: Vec<f32> = labels.iter().map(|l| p.layout_no_wrap((*l).to_owned(), font.clone(), INACTIVE_TEXT).size().x + PALETTE_TAB_PADDING).collect();
    let room = row.width() - 18.0;
    let total: f32 = natural.iter().sum();
    let widths: Vec<f32> = if total <= room || natural.len() < 2 {
        natural.clone()
    } else {
        // CS2 keeps the last tab whole and gives the others an equal share.
        let last = natural.last().copied().unwrap_or(0.0);
        let share = ((room - last) / (natural.len() - 1) as f32).max(PALETTE_TAB_PADDING + 4.0);
        natural.iter().enumerate().map(|(i, w)| if i + 1 == natural.len() { *w } else { w.min(share) }).collect()
    };
    let mut tabs = Vec::new();
    let mut drawn = Vec::new();
    let mut tx = row.left();
    for (i, (label, w)) in labels.iter().zip(&widths).enumerate() {
        let rect = Rect::from_min_size(pos2(tx, row.top()), vec2(*w, row.height()));
        let selected = active == Some(i);
        let mut job = egui::text::LayoutJob::simple_singleline((*label).to_owned(), font.clone(), if selected { Color32::BLACK } else { INACTIVE_TEXT });
        // A cut label runs up to the next tab's edge (CS2 clips it there, without an ellipsis).
        job.wrap = egui::text::TextWrapping { max_width: (w - TAB_INSET).max(4.0), max_rows: 1, break_anywhere: true, overflow_character: None };
        let galley = ui.fonts_mut(|f| f.layout_job(job));
        let response = ui.interact(rect, ui.scope_id().with(("cs2-well", i)), Sense::click_and_drag()).on_hover_text(*label);
        tabs.push(PaletteTab { rect, galley, active: selected, extra: 0.0 });
        drawn.push(WellTab { rect, response });
        tx += w;
    }
    paint_palette_tabs(&p, row, WELL_GREY, &tabs, true);
    // The well's bottom highlight, over the tab baselines.
    p.line_segment([well.left_bottom() + vec2(0.0, -0.5), well.right_bottom() + vec2(0.0, -0.5)], Stroke::new(1.0, HIGHLIGHT));
    WellDrawn { well, tabs: drawn, browse: b.clicked() }
}

/// Text on a selected list row: white on the Vista selection blue, as in CS2's palettes, so a
/// highlighted layer, channel or history state stays readable.
pub fn on_selection(t: &crate::theme::Tokens, selected: bool, normal: Color32) -> Color32 {
    if selected && crate::theme::is_cs2(t) { Color32::WHITE } else { normal }
}

// ------------------------------------------------------------------------------ work area

/// Where the document tabs start: right of the floating toolbox while it sits at the work
/// area's left edge, so it never covers a tab. A toolbox dragged elsewhere leaves `area` whole.
pub fn left_of_toolbox(ctx: &egui::Context, area: Rect) -> f32 {
    ctx.memory(|m| m.area_rect(egui::Id::unique("cs2-toolbox")))
        // Its default spot is just right of the vertical ruler.
        .filter(|tb| tb.intersects(area) && tb.left() <= area.left() + crate::canvas::rulers::RULER + 12.0)
        .map_or(area.left(), |tb| (tb.right() + 3.0).clamp(area.left(), area.right()))
}

/// Where the work area's content ends, left of the palettes parked against its right edge (the
/// CS2 default column), so the image, its scroll bars and Fit on Screen are not hidden under
/// them. `palettes` are the floating palette windows' `[x, y, width, height]`.
pub fn right_of_palettes(palettes: impl Iterator<Item = [f32; 4]>, area: Rect) -> f32 {
    palettes
        .map(|[x, y, w, h]| Rect::from_min_size(pos2(x, y), vec2(w, h)))
        .filter(|r| r.intersects(area) && r.right() >= area.right() - 8.0)
        .map(|r| r.left() - 3.0)
        .fold(area.right(), f32::min)
        .max(area.left())
}

// ------------------------------------------------------------------------------ keyboard

thread_local! {
    /// Whether the CS2 theme is showing, for code with no `egui::Context` at hand (the default
    /// shortcuts and tool letters). Set once per frame from the theme, on the UI thread; per
    /// thread so that tests running side by side don't see each other's theme.
    static ACTIVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn set_active(on: bool) {
    ACTIVE.with(|a| a.set(on));
}

pub fn active() -> bool {
    ACTIVE.with(std::cell::Cell::get)
}

/// Photoshop CS2's default keys where they differ from the modern defaults: (command, key, or
/// `""` for none). Ctrl+Z toggles Undo/Redo and Ctrl+Alt+Z steps back; channels are Ctrl+~ and
/// Ctrl+1…; Actual Pixels is Ctrl+Alt+0; Feather is Ctrl+Alt+D; Paste Into is Shift+Ctrl+V.
/// Commands CS2 did not have lose their modern keys.
pub const SHORTCUTS: &[(&str, &str)] = &[
    ("edit.toggleLastState", "Cmd+Z"),
    ("edit.undo", "Cmd+Alt+Z"),
    ("edit.redo", "Cmd+Shift+Z"),
    ("select.modify.feather", "Cmd+Alt+D"),
    ("view.actualPixels", "Cmd+Alt+0"),
    ("channel.target.composite", "Cmd+`"),
    ("select.loadLuminosity", "Cmd+Alt+`"),
    ("channel.target.slot3", "Cmd+1"),
    ("channel.target.slot4", "Cmd+2"),
    ("channel.target.slot5", "Cmd+3"),
    ("channel.target.slot6", "Cmd+4"),
    ("channel.target.slot7", "Cmd+5"),
    ("channel.target.slot8", "Cmd+6"),
    ("channel.target.slot9", "Cmd+7"),
    ("edit.pasteSpecial.pasteInto", "Cmd+Shift+V"),
    // Ctrl+F applies the last filter, as before Photoshop moved it to Ctrl+Alt+F for Search.
    ("filter.lastFilter", "Cmd+F"),
    ("edit.search", ""),
    ("edit.pasteSpecial.pasteInPlace", ""),
    ("select.selectAndMask", ""),
    ("filter.adaptiveWideAngle", ""),
    ("filter.cameraRaw", ""),
    ("filter.lensCorrection", ""),
    ("edit.contentAwareScale", ""),
    ("image.adjustments.blackWhite", ""),
];

/// A command's default key: CS2's while the CS2 theme shows, else `default`.
pub fn default_key<'a>(id: &str, default: Option<&'a str>) -> Option<&'a str> {
    if !active() {
        return default;
    }
    match SHORTCUTS.iter().find(|(c, _)| *c == id) {
        Some((_, "")) => None,
        Some((_, sc)) => Some(sc),
        None => default,
    }
}

/// A tool's CS2 letter where it differs from the modern one (`'\0'` = none): R is the Blur
/// group, N the Notes tool, K the Slice tools; tools CS2 did not have get no letter.
pub fn tool_key(tool: crate::state::Tool, modern: char) -> char {
    use crate::state::Tool as T;
    if !active() {
        return modern;
    }
    match tool {
        T::Blur | T::Sharpen | T::Smudge => 'R',
        T::Note => 'N',
        T::Slice | T::SliceSelect => 'K',
        T::RotateView
        | T::Count
        | T::Frame
        | T::Artboard
        | T::Remove
        | T::ContentAwareMove
        | T::QuickSelection
        | T::ObjectSelection
        | T::MixerBrush
        | T::PerspectiveCrop
        | T::CurvaturePen
        | T::Triangle
        | T::Star => '\0',
        _ => modern,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Tool;

    #[test]
    fn cs2_keys_apply_only_under_the_cs2_theme() {
        set_active(false);
        assert_eq!(crate::shortcuts::default_shortcut("edit.undo").as_deref(), Some("Cmd+Z"));
        assert_eq!(Tool::Blur.key(), '\0');
        set_active(true);
        // Ctrl+Z toggles Undo/Redo; Ctrl+Alt+Z steps back; Ctrl+Shift+Z steps forward.
        assert_eq!(crate::shortcuts::default_shortcut("edit.toggleLastState").as_deref(), Some("Cmd+Z"));
        assert_eq!(crate::shortcuts::default_shortcut("edit.undo").as_deref(), Some("Cmd+Alt+Z"));
        assert_eq!(crate::shortcuts::default_shortcut("edit.redo").as_deref(), Some("Cmd+Shift+Z"));
        assert_eq!(crate::shortcuts::default_shortcut("select.modify.feather").as_deref(), Some("Cmd+Alt+D"));
        assert_eq!(crate::shortcuts::default_shortcut("channel.target.slot3").as_deref(), Some("Cmd+1"));
        assert_eq!(crate::shortcuts::default_shortcut("edit.pasteSpecial.pasteInPlace"), None);
        assert_eq!(crate::shortcuts::default_shortcut("filter.lastFilter").as_deref(), Some("Cmd+F"));
        assert_eq!(Tool::Blur.key(), 'R');
        assert_eq!(Tool::Note.key(), 'N');
        assert_eq!(Tool::RotateView.key(), '\0');
        set_active(false);
    }

    #[test]
    fn the_canvas_stops_short_of_the_palette_column() {
        let area = Rect::from_min_max(pos2(0.0, 60.0), pos2(1456.0, 790.0));
        // The default column against the right edge, and one palette dragged into the middle.
        let column = [[1242.0, 62.0, 212.0, 132.0], [1242.0, 195.0, 212.0, 131.0], [600.0, 300.0, 212.0, 200.0]];
        assert_eq!(right_of_palettes(column.into_iter(), area), 1239.0);
        assert_eq!(right_of_palettes(std::iter::empty(), area), 1456.0);
    }

    /// The toolbox alone over a 1000 x 700 work area, in the CS2 theme.
    fn toolbox(app: crate::PhotocraftApp) -> egui_kittest::Harness<'static, crate::PhotocraftApp> {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1000.0, 700.0)).with_step_dt(1.0 / 60.0).build_ui_state(
            |ui, app: &mut crate::PhotocraftApp| {
                if ui.ctx().fonts(|f| f.families().contains(&egui::FontFamily::Name("medium".into()))) {
                    let work = ui.max_rect();
                    crate::panels::toolbar::floating_toolbox(app, ui.ctx(), work);
                }
            },
            app,
        );
        crate::PhotocraftApp::setup_context(&h.ctx, crate::theme::ThemeKind::Cs2);
        h.run_steps(3);
        h
    }

    fn toolbox_rect(h: &egui_kittest::Harness<'static, crate::PhotocraftApp>) -> Rect {
        h.ctx.memory(|m| m.area_rect(egui::Id::unique("cs2-toolbox"))).expect("the toolbox is shown")
    }

    #[test]
    fn the_toolbox_starts_beside_the_ruler_and_comes_back_where_it_was_left() {
        let new_app = || crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        // First launch: just right of the vertical ruler, which it must not cover.
        let mut h = toolbox(new_app());
        let first = toolbox_rect(&h);
        assert!(first.left() >= crate::canvas::rulers::RULER, "{first:?} covers the ruler");
        assert_eq!(h.state().ui.cs2_toolbox, None, "the default place is not saved as a choice");
        // Dragged by its title strip.
        h.state_mut().ui.tool = crate::state::Tool::RectMarquee;
        let grab = first.left_top() + egui::vec2(36.0, 4.0);
        h.event(egui::Event::PointerMoved(grab));
        h.step();
        h.event(egui::Event::PointerButton { pos: grab, button: egui::PointerButton::Primary, pressed: true, modifiers: egui::Modifiers::NONE });
        h.step();
        for i in 1..=10 {
            h.event(egui::Event::PointerMoved(grab + egui::vec2(40.0, 25.0) * i as f32));
            h.step();
        }
        h.event(egui::Event::PointerButton {
            pos: grab + egui::vec2(400.0, 250.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        });
        h.run_steps(3);
        let moved = toolbox_rect(&h);
        // Moving the toolbox picks no tool.
        assert_eq!(h.state().ui.tool, crate::state::Tool::RectMarquee, "the drag picked a tool");
        // It follows the pointer (to within egui's drag threshold).
        assert!((moved.min - (first.min + egui::vec2(400.0, 250.0))).length() < 12.0, "{first:?} -> {moved:?}");
        assert_eq!(h.state().ui.cs2_toolbox, Some([moved.min.x, moved.min.y]));
        // Saved with the panel layout, and restored from it by the next launch.
        let saved = crate::dock::snapshot(h.state());
        let mut next = new_app();
        crate::dock::apply(&mut next, &saved);
        let h = toolbox(next);
        assert!((toolbox_rect(&h).min - moved.min).length() < 1.0, "{:?} vs {moved:?}", toolbox_rect(&h));
    }

    #[test]
    fn every_cs2_key_names_a_real_command() {
        for (id, _) in SHORTCUTS {
            set_active(false);
            assert!(crate::shortcuts::default_shortcut(id).is_some(), "{id} has no modern default to replace");
        }
    }
}
