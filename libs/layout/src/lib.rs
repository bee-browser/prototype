mod flow;
mod spec;
mod style;

#[cfg(feature = "service")]
pub mod service;

use std::collections::BTreeMap;
use std::sync::Arc;

use euclid;
use euclid::num::Zero;

use crate::spec::*;
pub use crate::style::*;
use crate::flow::FlowContainer;

pub type Number = f32;

pub mod units {
    pub struct Pixel;
}

pub type Length = euclid::Length<Number, units::Pixel>;
pub(crate) const MAX_LENGTH: Length = Length::new(f32::MAX);

type Point2D = euclid::Point2D<Number, units::Pixel>;
type Size2D = euclid::Size2D<Number, units::Pixel>;
pub type Box2D = euclid::Box2D<Number, units::Pixel>;
pub type Rect = euclid::Rect<Number, units::Pixel>;
pub type Vector2D = euclid::Vector2D<Number, units::Pixel>;
pub type Transform2D = euclid::Transform2D<Number, units::Pixel, units::Pixel>;
pub type SideOffsets2D = euclid::SideOffsets2D<Number, units::Pixel>;

pub fn new_element(
    style: Arc<Style>, children: Vec<LayoutNodeHandle>, label: String) -> LayoutNodeHandle {
    let children = children.into_iter().map(|handle| handle.0).collect();
    LayoutNodeHandle(LayoutNodeRef::Element(
        Arc::new(LayoutElement::new(style, children, label))))
}

pub fn new_text(text: String, label: String) -> LayoutNodeHandle {
    LayoutNodeHandle(LayoutNodeRef::Text(
        Arc::new(LayoutText::new(text, label))))
}

pub fn build_visual_tree(layout_root: LayoutNodeHandle, width: usize, height: usize) -> VisualRoot {
    if let LayoutNodeRef::Element(ref element) = layout_root.0 {
        let width = Length::new(width as f32);
        let height = Length::new(height as f32);
        let content_box = Box2D::from_size(Size2D::from_lengths(width, height));

        let box_model = VisualBoxModel {
            style: element.style.clone(),
            geometry: BoxGeometry {
                margin_box: content_box.clone(),
                border_box: content_box.clone(),
                padding_box: content_box.clone(),
                content_box: content_box.clone(),
            },
        };

        let avail = AvailableSize {
            width: Some(width),
            height: Some(height),
        };

        let flow = element.build_flow(&avail);

        let layers = element.build_top_level_layers_for_children(&avail, &avail).into_vec();

        VisualRoot { width, height, box_model, flow, layers, }
    } else {
        unreachable!();  //<coverage:exclude/>
    }
}

#[derive(Clone)]
pub struct LayoutNodeHandle(LayoutNodeRef);  // opaque

impl LayoutNodeHandle {
    //<coverage:exclude>
    pub fn inspect<T>(&self, write: &mut T) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        self.0.inspect(write, 0)
    }
    //</coverage:exclude>
}

#[derive(Clone)]
enum LayoutNodeRef {
    Element(Arc<LayoutElement>),
    Text(Arc<LayoutText>),
}

impl LayoutNodeRef {
    fn maybe_element(&self) -> Option<&Arc<LayoutElement>> {
        match *self {
            Self::Element(ref element) => Some(element),
            _ => None,
        }
    }

    //<coverage:exclude>
    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        match *self {
            LayoutNodeRef::Element(ref element) => element.inspect(write, depth),
            LayoutNodeRef::Text(ref text) => text.inspect(write, depth),
        }
    }
    //</coverage:exclude>
}

struct LayoutElement {
    spec: Spec,
    style: Arc<Style>,
    children: Vec<LayoutNodeRef>,
    label: String,
}

impl LayoutElement {
    fn new(style: Arc<Style>, children: Vec<LayoutNodeRef>, label: String) -> Self {
        LayoutElement {
            spec: Spec::determine_from(&style),
            style,
            children,
            label,
        }
    }

    fn build_top_level_layers(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> VisualLayersMap {
        match self.style.positioning {
            PositioningScheme::Static =>
                self.build_top_level_layers_for_children(initial_avail, avail),
            PositioningScheme::Fixed =>
                self.build_top_level_layers_for_fixed(initial_avail),
            PositioningScheme::Absolute |
            PositioningScheme::Sticky =>  // TODO
                self.build_top_level_layers_for_other(initial_avail, avail),
            PositioningScheme::Relative =>
                self.build_top_level_layers_for_children(initial_avail, avail),  // TODO
        }
    }

    fn solve_box_geometry(&self, avail: &AvailableSize) -> SolvedBoxGeometry {
        let mut solver = BoxConstraintSolver::new(avail);
        solver
            .apply_style(&self.style)
            .solve_constraints();

        solver.geom
    }

    fn build_top_level_layers_for_fixed(
        &self,
        initial_avail: &AvailableSize,
    ) -> VisualLayersMap {
        let solved_geom = self.solve_box_geometry(initial_avail);

        // TODO: layout in-flow child elements.
        let flow = self.build_flow(&AvailableSize {
            width: solved_geom.width.value.clone(),
            height: solved_geom.height.value.clone(),
        });

        // TODO:
        // * update the position of the layer with the static position if it has not been solved.
        // * determine the height of the layer if it has not been solved.
        let box_model = VisualBoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
        };

        let new_avail = AvailableSize {
            width: Some(Length::new(box_model.padding_box().width())),
            height: Some(Length::new(box_model.padding_box().height())),
        };

        // The fixed layer always establishes a new stacking context.

        let (mut top_level_layers, child_layers) =
            self.build_layers_for_children(initial_avail, &new_avail);

        let stack_level = match self.style.layer.z_index {
            LayerZIndex::Auto => 0,
            LayerZIndex::Index(index) => index,
        };

        let layer = Arc::new(VisualLayer {
            box_model,
            stack_level,
            flow,
            child_layers: child_layers.into_vec(),
        });

        top_level_layers.push_front(layer);

        top_level_layers
    }

    fn build_top_level_layers_for_other(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> VisualLayersMap {
        let solved_geom = self.solve_box_geometry(avail);

        // TODO: layout in-flow child elements.
        let flow = self.build_flow(&AvailableSize {
            width: solved_geom.width.value.clone(),
            height: solved_geom.height.value.clone(),
        });

        // TODO:
        // * update the position of the layer with the static position if it has not been solved.
        // * determine the height of the layer if it has not been solved.
        let box_model = VisualBoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
        };

        let new_avail = AvailableSize {
            width: Some(Length::new(box_model.padding_box().width())),
            height: Some(Length::new(box_model.padding_box().height())),
        };

        match self.style.layer.z_index {
            LayerZIndex::Auto => {
                let mut top_level_layers = self.build_top_level_layers_for_children(
                    initial_avail, &new_avail);
                let layer = Arc::new(VisualLayer {
                    box_model,
                    stack_level: 0,
                    flow,
                    child_layers: vec![],
                });
                top_level_layers.push_front(layer);
                top_level_layers
            }
            LayerZIndex::Index(stack_level) => {
                let (mut top_level_layers, child_layers) = self.build_layers_for_children(
                    initial_avail, &new_avail);
                let layer = Arc::new(VisualLayer {
                    box_model,
                    stack_level,
                    flow,
                    child_layers: child_layers.into_vec(),
                });
                top_level_layers.push_front(layer);
                top_level_layers
            }
        }
    }

    fn build_top_level_layers_for_children(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> VisualLayersMap {
        self.children.iter()
            .filter_map(LayoutNodeRef::maybe_element)
            .map(|element| element.build_top_level_layers(
                initial_avail, avail))
            .fold(VisualLayersMap::new(), |mut acc, v| {
                acc.merge(v);
                acc
            })
    }

    fn build_layers(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> (VisualLayersMap, VisualLayersMap) {
        match self.style.positioning {
            PositioningScheme::Static =>
                self.build_layers_for_children(initial_avail, avail),
            PositioningScheme::Fixed =>
                self.build_layers_for_fixed(initial_avail),
            PositioningScheme::Absolute |
            PositioningScheme::Sticky =>  // TODO
                self.build_layers_for_other(initial_avail, avail),
            PositioningScheme::Relative =>
                self.build_layers_for_children(initial_avail, avail),  // TODO
        }
    }

    fn build_layers_for_fixed(
        &self,
        initial_avail: &AvailableSize,
    ) -> (VisualLayersMap, VisualLayersMap) {
        let solved_geom = self.solve_box_geometry(initial_avail);

        // TODO: layout in-flow child elements.
        let flow = self.build_flow(&AvailableSize {
            width: solved_geom.width.value.clone(),
            height: solved_geom.height.value.clone(),
        });

        // TODO:
        // * update the position of the layer with the static position if it has not been solved.
        // * determine the height of the layer if it has not been solved.
        let box_model = VisualBoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
        };

        let new_avail = AvailableSize {
            width: Some(Length::new(box_model.padding_box().width())),
            height: Some(Length::new(box_model.padding_box().height())),
        };

        let (mut top_level_layers, child_layers) =
            self.build_layers_for_children(initial_avail, &new_avail);

        let stack_level = match self.style.layer.z_index {
            LayerZIndex::Auto => 0,
            LayerZIndex::Index(index) => index,
        };

        let layer = Arc::new(VisualLayer {
            box_model,
            stack_level,
            flow,
            child_layers: child_layers.into_vec(),
        });

        top_level_layers.push_front(layer);
        (top_level_layers, VisualLayersMap::new())
    }

    fn build_layers_for_other(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> (VisualLayersMap, VisualLayersMap) {
        let solved_geom = self.solve_box_geometry(avail);

        // TODO: layout in-flow child elements.
        let flow = self.build_flow(&AvailableSize {
            width: solved_geom.width.value.clone(),
            height: solved_geom.height.value.clone(),
        });

        // TODO:
        // * update the position of the layer with the static position if it has not been solved.
        // * determine the height of the layer if it has not been solved.
        let box_model = VisualBoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
        };

        let new_avail = AvailableSize {
            width: Some(Length::new(box_model.padding_box().width())),
            height: Some(Length::new(box_model.padding_box().height())),
        };

        match self.style.layer.z_index {
            LayerZIndex::Auto => {
                let (top_level_layers, mut child_layers) =
                    self.build_layers_for_children(initial_avail, &new_avail);
                let layer = Arc::new(VisualLayer {
                    box_model,
                    stack_level: 0,
                    flow,
                    child_layers: vec![],
                });
                child_layers.push_front(layer);
                (top_level_layers, child_layers)
            }
            LayerZIndex::Index(stack_level) => {
                let (top_level_layers, child_layers) =
                    self.build_layers_for_children(initial_avail, &new_avail);
                let layer = Arc::new(VisualLayer {
                    box_model,
                    stack_level,
                    flow,
                    child_layers: child_layers.into_vec(),
                });
                let mut child_layers = VisualLayersMap::new();
                child_layers.push_back(layer);
                (top_level_layers, child_layers)
            }
        }
    }

    fn build_layers_for_children(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> (VisualLayersMap, VisualLayersMap) {
        self.children.iter()
            .filter_map(LayoutNodeRef::maybe_element)
            .map(|element| element.build_layers(
                initial_avail, avail))
            .fold((VisualLayersMap::new(), VisualLayersMap::new()), |mut acc, v| {
                acc.0.merge(v.0);
                acc.1.merge(v.1);
                acc
            })
    }

    //<coverage:exclude>
    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent=depth)?;
        for node_ref in self.children.iter() {
            node_ref.inspect(write, depth + 1)?;
        }
        Ok(())
    }
    //</coverage:exclude>
}

impl std::fmt::Display for LayoutElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "element: spec={:?} label=\"{}\"", self.spec, self.label)
    }
}

struct LayoutText {
    text: String,
    label: String,
}

impl LayoutText {
    fn new(text: String, label: String) -> Self {
        LayoutText { text, label }
    }

    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent=depth)
    }
}

impl std::fmt::Display for LayoutText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "text: text=\"{}\", label=\"{}\"",
               self.text.escape_debug().to_string(), self.label)
    }
}

// context node
pub struct VisualRoot {
    width: Length,
    height: Length,
    box_model: VisualBoxModel,
    // bounding box
    flow: Arc<FlowContainer>,
    layers: Vec<Arc<VisualLayer>>,
}

impl VisualRoot {
    pub fn inspect<T>(&self, write: &mut T) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "root: {:?}\n", self.box_model)?;

        for layer in self.layers.iter().filter(|layer| layer.stack_level < 0) {
            layer.inspect(write, 1)?;
        }

        self.flow.inspect(write, 1)?;

        for layer in self.layers.iter().filter(|layer| layer.stack_level >= 0) {
            layer.inspect(write, 1)?;
        }

        Ok(())
    }

    pub fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        renderer.start_render(self.width, self.height);

        // background and borders
        renderer.render_box(&self.box_model);

        // negative layers
        let v = self.box_model.padding_box().min.to_vector();
        renderer.translate_coord(v);
        for layer in self.layers.iter().filter(|layer| layer.stack_level < 0) {
            layer.render(renderer);
        }
        renderer.translate_coord(-v);

        // in-flow boxes
        let v = self.box_model.content_box().min.to_vector();
        renderer.translate_coord(v);
        self.flow.render(renderer);
        renderer.translate_coord(-v);

        // non-negative layers
        let v = self.box_model.padding_box().min.to_vector();
        renderer.translate_coord(v);
        for layer in self.layers.iter().filter(|layer| layer.stack_level >= 0) {
            layer.render(renderer);
        }
        renderer.translate_coord(-v);

        renderer.end_render();
    }
}

pub trait VisualRenderer {
    fn start_render(&mut self, width: Length, height: Length);
    fn end_render(&mut self);
    fn render_box(&mut self, model: &VisualBoxModel);
    fn translate_coord(&mut self, v: Vector2D);
}

pub struct VisualBoxModel {
    style: Arc<Style>,
    geometry: BoxGeometry,
}

pub struct BoxGeometry {
    margin_box: Box2D,
    border_box: Box2D,
    padding_box: Box2D,
    content_box: Box2D,
}

impl Default for BoxGeometry {
    fn default() -> Self {
        Self {
            margin_box: Box2D::zero(),
            border_box: Box2D::zero(),
            padding_box: Box2D::zero(),
            content_box: Box2D::zero(),
        }
    }
}

impl VisualBoxModel {
    pub fn margin_box(&self) -> &Box2D {
        &self.geometry.margin_box
    }

    pub fn border_box(&self) -> &Box2D {
        &self.geometry.border_box
    }

    pub fn padding_box(&self) -> &Box2D {
        &self.geometry.padding_box
    }

    pub fn content_box(&self) -> &Box2D {
        &self.geometry.content_box
    }

    pub fn background_color(&self) -> Color {
        self.style.background.color
    }

    pub fn border(&self) -> &BoxQuad<Border> {
        &self.style.box_model.border
    }
}

impl std::fmt::Debug for VisualBoxModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.border_box().to_rect())
    }
}

struct VisualLayer {
    box_model: VisualBoxModel,
    stack_level: i32,
    flow: Arc<FlowContainer>,
    child_layers: Vec<Arc<VisualLayer>>,
}

impl VisualLayer {
    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent=depth)?;

        for layer in self.child_layers.iter().filter(|layer| layer.stack_level < 0) {
            layer.inspect(write, depth + 1)?;
        }

        self.flow.inspect(write, depth + 1)?;

        for layer in self.child_layers.iter().filter(|layer| layer.stack_level >= 0) {
            layer.inspect(write, depth + 1)?;
        }

        Ok(())
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        // background and borders
        renderer.render_box(&self.box_model);

        // negative layers
        let v = self.box_model.padding_box().min.to_vector();
        renderer.translate_coord(v);
        for layer in self.child_layers.iter().filter(|layer| layer.stack_level < 0) {
            layer.render(renderer);
        }
        renderer.translate_coord(-v);

        // in-flow boxes
        let v = self.box_model.content_box().min.to_vector();
        renderer.translate_coord(v);
        self.flow.render(renderer);
        renderer.translate_coord(-v);

        // non-negative layers
        let v = self.box_model.padding_box().min.to_vector();
        renderer.translate_coord(v);
        for layer in self.child_layers.iter().filter(|layer| layer.stack_level >= 0) {
            layer.render(renderer);
        }
        renderer.translate_coord(-v);
    }
}

impl std::fmt::Display for VisualLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "layer: {:?}, stack_level={}", self.box_model, self.stack_level)
    }
}

struct BoxConstraintSolver {
    avail: AvailableSize,
    geom: SolvedBoxGeometry,
}

#[derive(Clone, Default)]
struct SolvedBoxGeometry {
    width: LengthWithRange,
    height: LengthWithRange,
    padding: BoxQuad<Length>,
    border: BoxQuad<Length>,
    margin: BoxQuad<Option<Length>>,
    offset: BoxQuad<Option<Length>>,
}

impl SolvedBoxGeometry {
    fn determine(self) -> BoxGeometry {
        let margin = self.margin.map(Option::unwrap);
        let offset = self.offset.map(Option::unwrap);

        let margin_min = Point2D::from_lengths(offset.get_left(), offset.get_top());
        let margin_max = margin_min
            + Size2D::from_lengths(
                margin.dw() + self.border.dw() + self.padding.dw() + self.width.value.unwrap(),
                margin.dh() + self.border.dh() + self.padding.dh() + self.height.value.unwrap());
        let margin_box = Box2D::new(margin_min, margin_max);
        let border_box = margin_box.inner_box(margin.into());
        let padding_box = border_box.inner_box(self.border.into());
        let content_box = padding_box.inner_box(self.padding.into());

        BoxGeometry { margin_box, border_box, padding_box, content_box }
    }
}

#[derive(Clone, Default)]
struct LengthWithRange {
    value: Option<Length>,
    min: Length,
    max: Length,
}

impl LengthWithRange {
    fn subtract(&mut self, delta: Length) {
        if delta == Length::zero() {
            return;
        }
        if let Some(ref mut value) = self.value {
            *value -= delta;
            if *value < Length::zero() {
                *value = Length::zero();
            }
        }
        if self.min != Length::zero() {
            self.min -= delta;
            if self.min < Length::zero() {
                self.min = Length::zero();
            }
        }
        if self.max != MAX_LENGTH {
            self.max -= delta;
            if self.max < Length::zero() {
                self.max = Length::zero();
            }
        }
    }
}

impl BoxConstraintSolver {
    fn new(avail: &AvailableSize) -> Self {
        BoxConstraintSolver {
            avail: avail.clone(),
            geom: Default::default(),
        }
    }

    fn apply_style(&mut self, style: &Style) -> &mut Self {
        self.geom.width.value = style.box_model.width.resolve(&self.avail.width);
        self.geom.width.min = style.box_model.min_width.resolve(&self.avail.width);
        self.geom.width.max = style.box_model.max_width.resolve(&self.avail.width);
        if self.geom.width.max < self.geom.width.min {
            self.geom.width.max = self.geom.width.min;
        }

        self.geom.height.value = style.box_model.height.resolve(&self.avail.height);
        self.geom.height.min = style.box_model.min_height.resolve(&self.avail.height);
        self.geom.height.max = style.box_model.max_height.resolve(&self.avail.height);
        if self.geom.height.max < self.geom.height.min {
            self.geom.height.max = self.geom.height.min;
        }

        self.geom.padding = style.box_model.padding.resolve(&self.avail);
        self.geom.border = style.box_model.border.resolve();
        self.geom.margin = style.box_model.margin.resolve(&self.avail);

        let (dw, dh) = match style.box_model.box_sizing {
            BoxSizing::ContentBox => (Length::zero(), Length::zero()),
            BoxSizing::PaddingBox => (self.geom.padding.dw(), self.geom.padding.dh()),
            BoxSizing::BorderBox => (self.geom.padding.dw() + self.geom.border.dw(),
                                     self.geom.padding.dh() + self.geom.border.dh()),
        };

        self.geom.width.subtract(dw);
        self.geom.height.subtract(dh);

        self.geom.offset = match style.positioning {
            PositioningScheme::Static | PositioningScheme::Relative => [
                Some(Length::zero()), Some(Length::zero()), None, Some(Length::zero())
            ].into(),
            _ => style.layer.offset.resolve(&self.avail),
        };

        self
    }

    fn solve_constraints(&mut self) -> &mut Self {
        if let Some(avail_width) = self.avail.width {
            self.solve_horizontal_constraints(
                avail_width - self.geom.border.dw() - self.geom.padding.dw());
        }
        if let Some(avail_height) = self.avail.height {
            self.solve_vertical_constraints(
                avail_height - self.geom.border.dh() - self.geom.padding.dh());
        }
        self
    }

    fn solve_horizontal_constraints(&mut self, avail_width: Length) {
        match (self.geom.width.value, self.geom.offset.get_left(), self.geom.offset.get_right()) {
            // none of the three is 'auto'
            (Some(width), Some(left), Some(right)) => {
                let remaining = avail_width - width - left - right;
                match (self.geom.margin.get_left(), self.geom.margin.get_right()) {
                    (None, None) => {
                        if remaining < Length::zero() {
                            // TODO: RTL
                            self.geom.margin.set_left(Some(Length::zero()));
                            self.geom.margin.set_right(Some(remaining));
                        } else {
                            // TODO: RTL
                            let half = remaining / 2.0;
                            self.geom.margin.set_left(Some(half));
                            self.geom.margin.set_right(Some(remaining - half));
                        }
                    }
                    (None, Some(right_margin)) => {
                        self.geom.margin.set_left(Some(remaining - right_margin));
                    }
                    (Some(left_margin), None) => {
                        self.geom.margin.set_right(Some(remaining - left_margin));
                    }
                    (Some(left_margin), Some(right_margin)) => {
                        // over-constrained.
                        // TODO: RTL
                        self.geom.offset.set_right(
                            Some(right + remaining - left_margin - right_margin));
                    }
                }
            }
            (Some(width), Some(left), None) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                self.geom.offset.set_right(
                    Some(avail_width - width - left - left_margin - right_margin));
            }
            (Some(width), None, Some(right)) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                self.geom.offset.set_left(
                    Some(avail_width - width - right - left_margin - right_margin));
            }
            (Some(width), None, None) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                // TODO: static-position, rtl
                let left = *self.geom.offset.left_mut().get_or_insert(Length::zero());
                self.geom.offset.set_right(
                    Some(avail_width - width - left - left_margin - right_margin));
            }
            (None, Some(left), Some(right)) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                self.geom.width.value =
                    Some(avail_width - left - right - left_margin - right_margin);
            }
            (None, Some(left), None) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                // TODO: shrink-to-fit
                let width = *self.geom.width.value.get_or_insert(Length::zero());
                self.geom.offset.set_right(
                    Some(avail_width - width - left - left_margin - right_margin));
            }
            (None, None, Some(right)) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                // TODO: shrink-to-fit
                let width = *self.geom.width.value.get_or_insert(Length::zero());
                self.geom.offset.set_left(
                    Some(avail_width - width - right - left_margin - right_margin));
            }
            (None, None, None) => {
                let left_margin = *self.geom.margin.left_mut().get_or_insert(Length::zero());
                let right_margin = *self.geom.margin.right_mut().get_or_insert(Length::zero());
                // TODO: shrink-to-fit
                let width = *self.geom.width.value.get_or_insert(Length::zero());
                // TODO: static-position, rtl
                let left = *self.geom.offset.left_mut().get_or_insert(Length::zero());
                self.geom.offset.set_right(
                    Some(avail_width - width - left - left_margin - right_margin));
            }
        }
    }

    fn solve_vertical_constraints(&mut self, avail_height: Length) {
        match (self.geom.height.value, self.geom.offset.get_top(), self.geom.offset.get_bottom()) {
            // none of the three is 'auto'
            (Some(height), Some(top), Some(bottom)) => {
                let remaining = avail_height - height - top - bottom;
                match (self.geom.margin.get_top(), self.geom.margin.get_bottom()) {
                    (None, None) => {
                        if remaining < Length::zero() {
                            self.geom.margin.set_top(Some(Length::zero()));
                            self.geom.margin.set_bottom(Some(remaining));
                        } else {
                            let half = remaining / 2.0;
                            self.geom.margin.set_top(Some(half));
                            self.geom.margin.set_bottom(Some(remaining - half));
                        }
                    }
                    (None, Some(bottom_margin)) => {
                        self.geom.margin.set_top(Some(remaining - bottom_margin));
                    }
                    (Some(top_margin), None) => {
                        self.geom.margin.set_bottom(Some(remaining - top_margin));
                    }
                    (Some(top_margin), Some(bottom_margin)) => {
                        // over-constrained.
                        self.geom.offset.set_bottom(
                            Some(bottom + remaining - top_margin - bottom_margin));
                    }
                }
            }
            (Some(height), Some(top), None) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                self.geom.offset.set_bottom(
                    Some(avail_height - height - top - top_margin - bottom_margin));
            }
            (Some(height), None, Some(bottom)) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                self.geom.offset.set_top(
                    Some(avail_height - height - bottom - top_margin - bottom_margin));
            }
            (Some(height), None, None) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                // TODO: static-position
                let top = *self.geom.offset.top_mut().get_or_insert(Length::zero());
                self.geom.offset.set_bottom(
                    Some(avail_height - height - top - top_margin - bottom_margin));
            }
            (None, Some(top), Some(bottom)) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                self.geom.height.value =
                    Some(avail_height - top - bottom - top_margin - bottom_margin);
            }
            (None, Some(top), None) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                // TODO: shrink-to-fit
                let height = *self.geom.height.value.get_or_insert(Length::zero());
                self.geom.offset.set_bottom(
                    Some(avail_height - height - top - top_margin - bottom_margin));
            }
            (None, None, Some(bottom)) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                // TODO: shrink-to-fit
                let height = *self.geom.height.value.get_or_insert(Length::zero());
                self.geom.offset.set_top(
                    Some(avail_height - height - bottom - top_margin - bottom_margin));
            }
            (None, None, None) => {
                let top_margin = *self.geom.margin.top_mut().get_or_insert(Length::zero());
                let bottom_margin = *self.geom.margin.bottom_mut().get_or_insert(Length::zero());
                // TODO: shrink-to-fit
                let height = *self.geom.height.value.get_or_insert(Length::zero());
                // TODO: static-position
                let top = *self.geom.offset.top_mut().get_or_insert(Length::zero());
                self.geom.offset.set_bottom(
                    Some(avail_height - height - top - top_margin - bottom_margin));
            }
        }
    }
}

// TODO: Inefficient in the memory point of view.
struct VisualLayersMap(BTreeMap<i32, Vec<Arc<VisualLayer>>>);

impl VisualLayersMap {
    fn new() -> Self {
        VisualLayersMap(BTreeMap::new())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push_back(&mut self, layer: Arc<VisualLayer>) {
        self.0
            .entry(layer.stack_level)
            .and_modify(|e| e.push(layer.clone()))
            .or_insert_with(|| vec![layer]);
    }

    fn push_front(&mut self, layer: Arc<VisualLayer>) {
        self.0
            .entry(layer.stack_level)
            .and_modify(|e| e.insert(0, layer.clone()))
            .or_insert_with(|| vec![layer]);
    }

    fn merge(&mut self, other: VisualLayersMap) {
        if other.is_empty() {
            return;
        }
        for (stack_level, mut layers) in other.0.into_iter() {
            self.0
                .entry(stack_level)
                .and_modify(|e| e.append(&mut layers))
                .or_insert(layers);
        }
    }

    fn into_vec(self) -> Vec<Arc<VisualLayer>> {
        // TODO: self.0.into_values().collect()
        let mut result = vec![];
        for (_, mut layers) in self.0.into_iter() {
            result.append(&mut layers);
        }
        result
    }
}
