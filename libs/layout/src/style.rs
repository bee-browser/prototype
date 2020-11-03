use std::ops::Add;

use euclid::num::Zero;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Number;
use crate::Length;
use crate::SideOffset2D;
use crate::MAX_LENGTH;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Style {
    pub schema: SchemaStyle,
    pub box_model: BoxModelStyle,
    pub background: BackgroundStyle,
    pub layer: LayerStyle,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SchemaStyle {
    pub node: VisualNodeType,
    pub container: VisualContainerType,
    pub positioning: PositioningScheme,
}

impl Default for SchemaStyle {
    fn default() -> Self {
        SchemaStyle {
            node: VisualNodeType::None,
            container: VisualContainerType::None,
            positioning: Default::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum VisualNodeType {
    None,
    Inline,
    Block,
    ListItem,
    TableCaption,
    TableHeaderGroup,
    TableFooterGroup,
    TableRowGroup,
    TableRow,
    TableColumnGroup,
    TableColumn,
    TableCell,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum VisualContainerType {
    None,
    Flow,
    FlowRoot,
    Table,
    TableRowGroup,
    TableRow,
    Widget,
    Flex,
    Grid,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PositioningScheme {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Default for PositioningScheme {
    fn default() -> Self {
        PositioningScheme::Static
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BoxModelStyle {
    pub box_sizing: BoxSizing,
    pub width: ContentSize,
    pub min_width: ContentMinSize,
    pub max_width: ContentMaxSize,
    pub height: ContentSize,
    pub min_height: ContentMinSize,
    pub max_height: ContentMaxSize,
    pub padding: BoxQuad<Padding>,
    pub border: BoxQuad<Border>,
    pub margin: BoxQuad<Margin>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
    PaddingBox,
}

impl Default for BoxSizing {
    fn default() -> Self {
        BoxSizing::ContentBox
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentSize {
    Auto,
    Pixel(Length),
    Scale(Number),
}

impl ContentSize {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Option<Length> {
        match (*self, *base) {
            (ContentSize::Pixel(px), _) => Some(px),
            (ContentSize::Scale(scale), Some(base)) => Some(base * scale),
            _ => None,
        }
    }
}

impl Default for ContentSize {
    fn default() -> Self {
        ContentSize::Auto
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentMinSize {
    Pixel(Length),
    Scale(Number),
}

impl ContentMinSize {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Length {
        match (*self, *base) {
            (ContentMinSize::Pixel(px), _) => px,
            (ContentMinSize::Scale(scale), Some(base)) => base * scale,
            _ => Length::zero(),
        }
    }
}

impl Default for ContentMinSize {
    fn default() -> Self {
        ContentMinSize::Pixel(Default::default())
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentMaxSize {
    None,
    Pixel(Length),
    Scale(Number),
}

impl ContentMaxSize {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Length {
        match (*self, *base) {
            (ContentMaxSize::Pixel(px), _) => px,
            (ContentMaxSize::Scale(scale), Some(base)) => base * scale,
            _ => MAX_LENGTH,
        }
    }
}

impl Default for ContentMaxSize {
    fn default() -> Self {
        ContentMaxSize::None
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Padding {
    Pixel(Length),
    Scale(Number),
}

impl Padding {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Length {
        match (*self, *base) {
            (Padding::Pixel(px), _) => px,
            (Padding::Scale(scale), Some(base)) => base * scale,
            _ => Length::zero(),
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Padding::Pixel(Default::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BorderStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl BorderStyle {
    #[inline]
    pub fn is_visible(&self) -> bool {
        match *self {
            BorderStyle::None | BorderStyle::Hidden => false,
            _ => true,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::None
    }
}

impl std::fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => write!(f, "none"),
            Self::Hidden => write!(f, "hidden"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Solid => write!(f, "solid"),
            Self::Double => write!(f, "double"),
            Self::Groove => write!(f, "groove"),
            Self::Ridge => write!(f, "ridge"),
            Self::Inset => write!(f, "inset"),
            Self::Outset => write!(f, "outset"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Border {
    pub style: BorderStyle,
    pub width: Length,
    pub color: Color,
}

impl Border {
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.style.is_visible() && self.width > Length::zero() && !self.color.is_transparent()
    }

    #[inline]
    pub fn resolve(&self) -> Length {
        match self.style {
            BorderStyle::None => Length::zero(),
            _ => self.width,
        }
    }
}

impl BoxQuad<Border> {
    pub fn is_visible(&self) -> bool {
        self.0.iter().any(|style| style.is_visible())
    }
}

impl Into<SideOffset2D> for BoxQuad<Border> {
    fn into(self) -> SideOffset2D {
        SideOffset2D::from_lengths(
            self.top().width, self.right().width, self.bottom().width, self.left().width)
    }
}

impl std::fmt::Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.width, self.color)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Color(u8, u8, u8, u8);

impl Color {
    #[inline]
    pub fn red(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.2
    }

    #[inline]
    pub fn alpha(&self) -> u8 {
        self.3
    }

    pub fn is_transparent(&self) -> bool {
        self.alpha() == 0
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha() == 255
    }

    pub fn is_translucent(&self) -> bool {
        !self.is_transparent() && !self.is_opaque()
    }
}

impl Default for Color {
    fn default() -> Self {
        Color(0, 0, 0, 0)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.red(), self.green(), self.blue(), self.alpha())
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Margin {
    Auto,
    Pixel(Length),
    Scale(Number),
}

impl Margin {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Option<Length> {
        match (*self, *base) {
            (Margin::Pixel(px), _) => Some(px),
            (Margin::Scale(scale), Some(base)) => Some(base * scale),
            _ => None,
        }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Margin::Auto
    }
}

#[derive(Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BoxQuad<T>([T; 4]);

impl<T> BoxQuad<T>
where
    T: Copy,
{
    #[inline]
    pub fn new(v: T) -> Self {
        BoxQuad([v; 4])
    }

    #[inline]
    pub fn top(&self) -> T {
        self.0[0]
    }

    #[inline]
    pub fn top_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    #[inline]
    pub fn set_top(&mut self, value: T) {
        self.0[0] = value;
    }

    #[inline]
    pub fn right(&self) -> T {
        self.0[1]
    }

    #[inline]
    pub fn right_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    #[inline]
    pub fn set_right(&mut self, value: T) {
        self.0[1] = value;
    }

    #[inline]
    pub fn bottom(&self) -> T {
        self.0[2]
    }

    #[inline]
    pub fn bottom_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }

    #[inline]
    pub fn set_bottom(&mut self, value: T) {
        self.0[2] = value;
    }

    #[inline]
    pub fn left(&self) -> T {
        self.0[3]
    }

    #[inline]
    pub fn left_mut(&mut self) -> &mut T {
        &mut self.0[3]
    }

    #[inline]
    pub fn set_left(&mut self, value: T) {
        self.0[3] = value;
    }
}

impl<T> BoxQuad<T>
where
    T: Copy + Add<Output = T>
{
    #[inline]
    pub fn dw(&self) -> T {
        self.left() + self.right()
    }

    #[inline]
    pub fn dh(&self) -> T {
        self.top() + self.bottom()
    }
}

impl<T> std::fmt::Display for BoxQuad<T>
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl<T> std::fmt::Debug for BoxQuad<T>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}, {:?}]", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl BoxQuad<Padding> {
    #[inline]
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Length> {
        BoxQuad([
            self.0[0].resolve(&avail.width),
            self.0[1].resolve(&avail.width),
            self.0[2].resolve(&avail.width),
            self.0[3].resolve(&avail.width),
        ])
    }
}

impl BoxQuad<Border> {
    #[inline]
    pub fn resolve(&self) -> BoxQuad<Length> {
        BoxQuad([
            self.0[0].resolve(),
            self.0[1].resolve(),
            self.0[2].resolve(),
            self.0[3].resolve(),
        ])
    }
}

impl BoxQuad<Margin> {
    #[inline]
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Option<Length>> {
        BoxQuad([
            self.0[0].resolve(&avail.width),
            self.0[1].resolve(&avail.width),
            self.0[2].resolve(&avail.width),
            self.0[3].resolve(&avail.width),
        ])
    }
}

impl BoxQuad<LayerOffset> {
    #[inline]
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Option<Length>> {
        BoxQuad([
            self.0[0].resolve(&avail.height),
            self.0[1].resolve(&avail.width),
            self.0[2].resolve(&avail.height),
            self.0[3].resolve(&avail.width),
        ])
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BackgroundStyle {
    pub color: Color,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LayerStyle {
    pub offset: BoxQuad<LayerOffset>,
    pub z_index: LayerZIndex,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LayerOffset {
    Auto,
    Pixel(Length),
    Scale(Number),
}

impl LayerOffset {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Option<Length> {
        match (*self, *base) {
            (LayerOffset::Pixel(value), _) => Some(value),
            (LayerOffset::Scale(scale), Some(base)) => Some(base * scale),
            _ => None,
        }
    }
}

impl Default for LayerOffset {
    fn default() -> Self {
        LayerOffset::Auto
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LayerZIndex {
    Auto,
    Index(i32),
}

impl Default for LayerZIndex {
    fn default() -> Self {
        LayerZIndex::Auto
    }
}

#[derive(Clone)]
pub struct AvailableSize {
    pub width: Option<Length>,
    pub height: Option<Length>,
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use assert_matches::assert_matches;

    #[test]
    fn test_border_style_serde() {
        let pairs = [
            (BorderStyle::None, r#""none""#),
            (BorderStyle::Hidden, r#""hidden""#),
            (BorderStyle::Dotted, r#""dotted""#),
            (BorderStyle::Dashed, r#""dashed""#),
            (BorderStyle::Solid, r#""solid""#),
            (BorderStyle::Double, r#""double""#),
            (BorderStyle::Groove, r#""groove""#),
            (BorderStyle::Ridge, r#""ridge""#),
            (BorderStyle::Inset, r#""inset""#),
            (BorderStyle::Outset, r#""outset""#),
        ];
        for pair in pairs.iter() {
            let result = assert_matches!(serde_json::to_string(&pair.0), Ok(v) => v);
            assert_eq!(result, pair.1);

            let result: BorderStyle =
                assert_matches!(serde_json::from_str(&pair.1), Ok(v) => v);
            assert_eq!(result, pair.0);
        }
    }

    #[test]
    fn test_border_serde() {
        let border = Border {
            style: BorderStyle::Solid,
            width: Length::new(10.0),
            color: Color(0, 0, 0, 0),
        };

        let json = r#"{"style":"solid","width":10.0,"color":[0,0,0,0]}"#;

        let result = assert_matches!(serde_json::to_string(&border), Ok(v) => v);
        assert_eq!(result, json);

        let result: Border = assert_matches!(serde_json::from_str(&json), Ok(v) => v);
        assert_eq!(result, border);
    }

    #[test]
    fn test_border_quad_serde() {
        let value = BoxQuad::new(Border {
            style: BorderStyle::Solid,
            width: Length::new(10.0),
            color: Color(0, 0, 0, 0),
        });

        let json = r#"[{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]}]"#;

        let result = assert_matches!(serde_json::to_string(&value), Ok(v) => v);
        assert_eq!(result, json);

        let result: BoxQuad<Border> = assert_matches!(serde_json::from_str(&json), Ok(v) => v);
        assert_eq!(result, value);
    }
}
