// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// bee-tools-codegen.js --no-escape --input-stdin mod.rs.hbs

//<coverage:exclude>

use std::fmt::Debug;

use phf::phf_map;
use phf::Map;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LocalName {
    A,
    Abbr,
    Address,
    Area,
    Article,
    Aside,
    Audio,
    B,
    Base,
    Bdi,
    Bdo,
    Blockquote,
    Body,
    Br,
    Button,
    Canvas,
    Caption,
    Cite,
    Code,
    Col,
    Colgroup,
    Data,
    Datalist,
    Dd,
    Del,
    Details,
    Dfn,
    Dialog,
    Div,
    Dl,
    Dt,
    Em,
    Embed,
    Fieldset,
    Figcaption,
    Figure,
    Footer,
    Form,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Hr,
    Html,
    I,
    Iframe,
    Img,
    Input,
    Ins,
    Kbd,
    Label,
    Legend,
    Li,
    Link,
    Main,
    Map,
    Mark,
    Menu,
    Meta,
    Meter,
    Nav,
    Noscript,
    Object,
    Ol,
    Optgroup,
    Option,
    Output,
    P,
    Picture,
    Pre,
    Progress,
    Q,
    Rp,
    Rt,
    Ruby,
    S,
    Samp,
    Sarcasm,
    Script,
    Section,
    Select,
    Slot,
    Small,
    Source,
    Span,
    Strong,
    Style,
    Sub,
    Summary,
    Sup,
    Table,
    Tbody,
    Td,
    Template,
    Textarea,
    Tfoot,
    Th,
    Thead,
    Time,
    Title,
    Tr,
    Track,
    U,
    Ul,
    Var,
    Video,
    Wbr,
    Acronym,
    Applet,
    Basefont,
    Bgsound,
    Big,
    Blink,
    Center,
    Content,
    Dir,
    Font,
    Frame,
    Frameset,
    Hgroup,
    Isindex,
    Keygen,
    Listing,
    Marquee,
    Menuitem,
    Multicol,
    Nextid,
    Nobr,
    Noembed,
    Noframes,
    Param,
    Plaintext,
    Rb,
    Rtc,
    Shadow,
    Spacer,
    Strike,
    Tt,
    Xmp,
    Math,
    AnnotationXml,
    Mi,
    Mo,
    Mn,
    Ms,
    Mtext,
    Svg,
    ForeignObject,
    Desc,
    AltGlyph,
    AltGlyphDef,
    AltGlyphItem,
    AnimateColor,
    AnimateMotion,
    AnimateTransform,
    ClipPath,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    GlyphRef,
    LinearGradient,
    RadialGradient,
    TextPath,
    Mglyph,
    Malignmark,
    Unknown,
}

impl LocalName {
    pub fn lookup(s: &str) -> Self {
        LOCAL_NAMES.get(s).cloned().unwrap_or(Self::Unknown)
    }

    pub fn name(self) -> &'static str {
        if self == Self::Unknown {
            return "UNKNOWN";
        }
        DATA[self.as_index()].name
    }

    pub fn is_special(self) -> bool {
        if self == Self::Unknown {
            return false;
        }
        match DATA[self.as_index()].category {
            Category::Special => true,
            _ => false,
        }
    }

    pub fn is_formatting(self) -> bool {
        if self == Self::Unknown {
            return false;
        }
        match DATA[self.as_index()].category {
            Category::Formatting => true,
            _ => false,
        }
    }

    fn as_index(self) -> usize {
        match self {
            Self::Unknown => panic!(),
            _ => self as usize,
        }
    }
}

impl Debug for LocalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "UNKNOWN"),
            _ => write!(f, "{}", self.name()),
        }
    }
}

// models

struct LocalNameData {
    name: &'static str,
    category: Category,
    obsolete: bool,
}

enum Category {
    Special,
    Formatting,
    Ordinary,
}

static LOCAL_NAMES: Map<&'static str, LocalName> = phf_map! {
    "a" => LocalName::A,
    "abbr" => LocalName::Abbr,
    "address" => LocalName::Address,
    "area" => LocalName::Area,
    "article" => LocalName::Article,
    "aside" => LocalName::Aside,
    "audio" => LocalName::Audio,
    "b" => LocalName::B,
    "base" => LocalName::Base,
    "bdi" => LocalName::Bdi,
    "bdo" => LocalName::Bdo,
    "blockquote" => LocalName::Blockquote,
    "body" => LocalName::Body,
    "br" => LocalName::Br,
    "button" => LocalName::Button,
    "canvas" => LocalName::Canvas,
    "caption" => LocalName::Caption,
    "cite" => LocalName::Cite,
    "code" => LocalName::Code,
    "col" => LocalName::Col,
    "colgroup" => LocalName::Colgroup,
    "data" => LocalName::Data,
    "datalist" => LocalName::Datalist,
    "dd" => LocalName::Dd,
    "del" => LocalName::Del,
    "details" => LocalName::Details,
    "dfn" => LocalName::Dfn,
    "dialog" => LocalName::Dialog,
    "div" => LocalName::Div,
    "dl" => LocalName::Dl,
    "dt" => LocalName::Dt,
    "em" => LocalName::Em,
    "embed" => LocalName::Embed,
    "fieldset" => LocalName::Fieldset,
    "figcaption" => LocalName::Figcaption,
    "figure" => LocalName::Figure,
    "footer" => LocalName::Footer,
    "form" => LocalName::Form,
    "h1" => LocalName::H1,
    "h2" => LocalName::H2,
    "h3" => LocalName::H3,
    "h4" => LocalName::H4,
    "h5" => LocalName::H5,
    "h6" => LocalName::H6,
    "head" => LocalName::Head,
    "header" => LocalName::Header,
    "hr" => LocalName::Hr,
    "html" => LocalName::Html,
    "i" => LocalName::I,
    "iframe" => LocalName::Iframe,
    "img" => LocalName::Img,
    "input" => LocalName::Input,
    "ins" => LocalName::Ins,
    "kbd" => LocalName::Kbd,
    "label" => LocalName::Label,
    "legend" => LocalName::Legend,
    "li" => LocalName::Li,
    "link" => LocalName::Link,
    "main" => LocalName::Main,
    "map" => LocalName::Map,
    "mark" => LocalName::Mark,
    "menu" => LocalName::Menu,
    "meta" => LocalName::Meta,
    "meter" => LocalName::Meter,
    "nav" => LocalName::Nav,
    "noscript" => LocalName::Noscript,
    "object" => LocalName::Object,
    "ol" => LocalName::Ol,
    "optgroup" => LocalName::Optgroup,
    "option" => LocalName::Option,
    "output" => LocalName::Output,
    "p" => LocalName::P,
    "picture" => LocalName::Picture,
    "pre" => LocalName::Pre,
    "progress" => LocalName::Progress,
    "q" => LocalName::Q,
    "rp" => LocalName::Rp,
    "rt" => LocalName::Rt,
    "ruby" => LocalName::Ruby,
    "s" => LocalName::S,
    "samp" => LocalName::Samp,
    "sarcasm" => LocalName::Sarcasm,
    "script" => LocalName::Script,
    "section" => LocalName::Section,
    "select" => LocalName::Select,
    "slot" => LocalName::Slot,
    "small" => LocalName::Small,
    "source" => LocalName::Source,
    "span" => LocalName::Span,
    "strong" => LocalName::Strong,
    "style" => LocalName::Style,
    "sub" => LocalName::Sub,
    "summary" => LocalName::Summary,
    "sup" => LocalName::Sup,
    "table" => LocalName::Table,
    "tbody" => LocalName::Tbody,
    "td" => LocalName::Td,
    "template" => LocalName::Template,
    "textarea" => LocalName::Textarea,
    "tfoot" => LocalName::Tfoot,
    "th" => LocalName::Th,
    "thead" => LocalName::Thead,
    "time" => LocalName::Time,
    "title" => LocalName::Title,
    "tr" => LocalName::Tr,
    "track" => LocalName::Track,
    "u" => LocalName::U,
    "ul" => LocalName::Ul,
    "var" => LocalName::Var,
    "video" => LocalName::Video,
    "wbr" => LocalName::Wbr,
    "acronym" => LocalName::Acronym,
    "applet" => LocalName::Applet,
    "basefont" => LocalName::Basefont,
    "bgsound" => LocalName::Bgsound,
    "big" => LocalName::Big,
    "blink" => LocalName::Blink,
    "center" => LocalName::Center,
    "content" => LocalName::Content,
    "dir" => LocalName::Dir,
    "font" => LocalName::Font,
    "frame" => LocalName::Frame,
    "frameset" => LocalName::Frameset,
    "hgroup" => LocalName::Hgroup,
    "isindex" => LocalName::Isindex,
    "keygen" => LocalName::Keygen,
    "listing" => LocalName::Listing,
    "marquee" => LocalName::Marquee,
    "menuitem" => LocalName::Menuitem,
    "multicol" => LocalName::Multicol,
    "nextid" => LocalName::Nextid,
    "nobr" => LocalName::Nobr,
    "noembed" => LocalName::Noembed,
    "noframes" => LocalName::Noframes,
    "param" => LocalName::Param,
    "plaintext" => LocalName::Plaintext,
    "rb" => LocalName::Rb,
    "rtc" => LocalName::Rtc,
    "shadow" => LocalName::Shadow,
    "spacer" => LocalName::Spacer,
    "strike" => LocalName::Strike,
    "tt" => LocalName::Tt,
    "xmp" => LocalName::Xmp,
    "math" => LocalName::Math,
    "annotation-xml" => LocalName::AnnotationXml,
    "mi" => LocalName::Mi,
    "mo" => LocalName::Mo,
    "mn" => LocalName::Mn,
    "ms" => LocalName::Ms,
    "mtext" => LocalName::Mtext,
    "svg" => LocalName::Svg,
    "foreignobject" => LocalName::ForeignObject,
    "desc" => LocalName::Desc,
    "altglyph" => LocalName::AltGlyph,
    "altglyphdef" => LocalName::AltGlyphDef,
    "altglyphitem" => LocalName::AltGlyphItem,
    "animatecolor" => LocalName::AnimateColor,
    "animatemotion" => LocalName::AnimateMotion,
    "animatetransform" => LocalName::AnimateTransform,
    "clippath" => LocalName::ClipPath,
    "feblend" => LocalName::FeBlend,
    "fecolormatrix" => LocalName::FeColorMatrix,
    "fecomponenttransfer" => LocalName::FeComponentTransfer,
    "fecomposite" => LocalName::FeComposite,
    "feconvolvematrix" => LocalName::FeConvolveMatrix,
    "fediffuselighting" => LocalName::FeDiffuseLighting,
    "fedisplacementmap" => LocalName::FeDisplacementMap,
    "fedistantlight" => LocalName::FeDistantLight,
    "fedropshadow" => LocalName::FeDropShadow,
    "feflood" => LocalName::FeFlood,
    "fefunca" => LocalName::FeFuncA,
    "fefuncb" => LocalName::FeFuncB,
    "fefuncg" => LocalName::FeFuncG,
    "fefuncr" => LocalName::FeFuncR,
    "fegaussianblur" => LocalName::FeGaussianBlur,
    "feimage" => LocalName::FeImage,
    "femerge" => LocalName::FeMerge,
    "femergenode" => LocalName::FeMergeNode,
    "femorphology" => LocalName::FeMorphology,
    "feoffset" => LocalName::FeOffset,
    "fepointlight" => LocalName::FePointLight,
    "fespecularlighting" => LocalName::FeSpecularLighting,
    "fespotlight" => LocalName::FeSpotLight,
    "fetile" => LocalName::FeTile,
    "feturbulence" => LocalName::FeTurbulence,
    "glyphref" => LocalName::GlyphRef,
    "lineargradient" => LocalName::LinearGradient,
    "radialgradient" => LocalName::RadialGradient,
    "textpath" => LocalName::TextPath,
    "mglyph" => LocalName::Mglyph,
    "malignmark" => LocalName::Malignmark,
};

macro_rules! data {
    ($name:literal, $category:ident) => {
        LocalNameData {
            name: $name,
            category: Category::$category,
            obsolete: false,
        }
    };
    (obsolete: $name:literal, $category:ident) => {
        LocalNameData {
            name: $name,
            category: Category::$category,
            obsolete: true,
        }
    };
}

const DATA: [LocalNameData; 191] = [
    data!["a", Formatting],
    data!["abbr", Ordinary],
    data!["address", Special],
    data!["area", Special],
    data!["article", Special],
    data!["aside", Special],
    data!["audio", Ordinary],
    data!["b", Formatting],
    data!["base", Special],
    data!["bdi", Ordinary],
    data!["bdo", Ordinary],
    data!["blockquote", Special],
    data!["body", Special],
    data!["br", Special],
    data!["button", Special],
    data!["canvas", Ordinary],
    data!["caption", Special],
    data!["cite", Ordinary],
    data!["code", Formatting],
    data!["col", Special],
    data!["colgroup", Special],
    data!["data", Ordinary],
    data!["datalist", Ordinary],
    data!["dd", Special],
    data!["del", Ordinary],
    data!["details", Special],
    data!["dfn", Ordinary],
    data!["dialog", Ordinary],
    data!["div", Special],
    data!["dl", Special],
    data!["dt", Special],
    data!["em", Formatting],
    data!["embed", Special],
    data!["fieldset", Special],
    data!["figcaption", Special],
    data!["figure", Special],
    data!["footer", Special],
    data!["form", Special],
    data!["h1", Special],
    data!["h2", Special],
    data!["h3", Special],
    data!["h4", Special],
    data!["h5", Special],
    data!["h6", Special],
    data!["head", Special],
    data!["header", Special],
    data!["hr", Special],
    data!["html", Special],
    data!["i", Formatting],
    data!["iframe", Special],
    data!["img", Special],
    data!["input", Special],
    data!["ins", Ordinary],
    data!["kbd", Ordinary],
    data!["label", Ordinary],
    data!["legend", Ordinary],
    data!["li", Special],
    data!["link", Special],
    data!["main", Special],
    data!["map", Ordinary],
    data!["mark", Ordinary],
    data!["menu", Special],
    data!["meta", Special],
    data!["meter", Ordinary],
    data!["nav", Special],
    data!["noscript", Special],
    data!["object", Special],
    data!["ol", Special],
    data!["optgroup", Ordinary],
    data!["option", Ordinary],
    data!["output", Ordinary],
    data!["p", Special],
    data!["picture", Ordinary],
    data!["pre", Special],
    data!["progress", Ordinary],
    data!["q", Ordinary],
    data!["rp", Ordinary],
    data!["rt", Ordinary],
    data!["ruby", Ordinary],
    data!["s", Formatting],
    data!["samp", Ordinary],
    data!["sarcasm", Ordinary],
    data!["script", Special],
    data!["section", Special],
    data!["select", Special],
    data!["slot", Ordinary],
    data!["small", Formatting],
    data!["source", Special],
    data!["span", Ordinary],
    data!["strong", Formatting],
    data!["style", Special],
    data!["sub", Ordinary],
    data!["summary", Special],
    data!["sup", Ordinary],
    data!["table", Special],
    data!["tbody", Special],
    data!["td", Special],
    data!["template", Special],
    data!["textarea", Special],
    data!["tfoot", Special],
    data!["th", Special],
    data!["thead", Special],
    data!["time", Ordinary],
    data!["title", Special],
    data!["tr", Special],
    data!["track", Special],
    data!["u", Formatting],
    data!["ul", Special],
    data!["var", Ordinary],
    data!["video", Ordinary],
    data!["wbr", Special],
    data![obsolete: "acronym", Ordinary],
    data![obsolete: "applet", Special],
    data![obsolete: "basefont", Special],
    data![obsolete: "bgsound", Special],
    data![obsolete: "big", Formatting],
    data![obsolete: "blink", Ordinary],
    data![obsolete: "center", Special],
    data![obsolete: "content", Ordinary],
    data![obsolete: "dir", Special],
    data![obsolete: "font", Formatting],
    data![obsolete: "frame", Special],
    data![obsolete: "frameset", Special],
    data![obsolete: "hgroup", Special],
    data![obsolete: "isindex", Ordinary],
    data![obsolete: "keygen", Special],
    data![obsolete: "listing", Special],
    data![obsolete: "marquee", Special],
    data![obsolete: "menuitem", Ordinary],
    data![obsolete: "multicol", Ordinary],
    data![obsolete: "nextid", Ordinary],
    data![obsolete: "nobr", Formatting],
    data![obsolete: "noembed", Special],
    data![obsolete: "noframes", Special],
    data![obsolete: "param", Special],
    data![obsolete: "plaintext", Special],
    data![obsolete: "rb", Ordinary],
    data![obsolete: "rtc", Ordinary],
    data![obsolete: "shadow", Ordinary],
    data![obsolete: "spacer", Ordinary],
    data![obsolete: "strike", Formatting],
    data![obsolete: "tt", Formatting],
    data![obsolete: "xmp", Special],
    data!["math", Ordinary],
    data!["annotation-xml", Special],
    data!["mi", Special],
    data!["mo", Special],
    data!["mn", Special],
    data!["ms", Special],
    data!["mtext", Special],
    data!["svg", Ordinary],
    data!["foreignObject", Special],
    data!["desc", Special],
    data!["altGlyph", Ordinary],
    data!["altGlyphDef", Ordinary],
    data!["altGlyphItem", Ordinary],
    data!["animateColor", Ordinary],
    data!["animateMotion", Ordinary],
    data!["animateTransform", Ordinary],
    data!["clipPath", Ordinary],
    data!["feBlend", Ordinary],
    data!["feColorMatrix", Ordinary],
    data!["feComponentTransfer", Ordinary],
    data!["feComposite", Ordinary],
    data!["feConvolveMatrix", Ordinary],
    data!["feDiffuseLighting", Ordinary],
    data!["feDisplacementMap", Ordinary],
    data!["feDistantLight", Ordinary],
    data!["feDropShadow", Ordinary],
    data!["feFlood", Ordinary],
    data!["feFuncA", Ordinary],
    data!["feFuncB", Ordinary],
    data!["feFuncG", Ordinary],
    data!["feFuncR", Ordinary],
    data!["feGaussianBlur", Ordinary],
    data!["feImage", Ordinary],
    data!["feMerge", Ordinary],
    data!["feMergeNode", Ordinary],
    data!["feMorphology", Ordinary],
    data!["feOffset", Ordinary],
    data!["fePointLight", Ordinary],
    data!["feSpecularLighting", Ordinary],
    data!["feSpotLight", Ordinary],
    data!["feTile", Ordinary],
    data!["feTurbulence", Ordinary],
    data!["glyphRef", Ordinary],
    data!["linearGradient", Ordinary],
    data!["radialGradient", Ordinary],
    data!["textPath", Ordinary],
    data!["mglyph", Ordinary],
    data!["malignmark", Ordinary],
];

//</coverage:exclude>
