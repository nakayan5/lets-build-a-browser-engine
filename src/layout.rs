// https://limpet.net/mbrubeck/2014/09/08/toy-layout-engine-5-boxes.html
// @see https://developer.chrome.com/blog/inside-browser-part3?hl=ja

use crate::css::Value;
use crate::layout::BoxType::BlockNode;
use crate::layout::BoxType::InlineNode;
use crate::layout::Value::Keyword;
use crate::style::StyledNode;

// CSS box model. All sizes are in px.
struct Dimensions {
    // Position of the content area relative to the document origin:
    content: Rect,

    // Surrounding edges:
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

// Block and Inline Layout
/**
 * 各ボックスには、ブロックの子だけ、またはインラインの子だけを含める必要があります。
 * DOM要素にブロックの子とインラインの子が混在している場合、レイアウト・エンジンは2つのタイプを分けるために匿名ボックスを挿入します。
 * (これらのボックスはDOMツリーのノードに関連付けられていないので「匿名」です)。
 */
struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}
enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}
/**
 * レイアウト・ツリーを構築するには、各DOMノードのdisplayプロパティを調べる必要があります。
 * ノードのdisplay値を取得するコードをstyleモジュールに追加しました。
 * 指定された値がない場合は、初期値の「inline」を返します。
 * ノードの display プロパティが「none」に設定されている場合、そのノードはレイアウト ツリーに含まれません。
 */
enum Display {
    Inline,
    Block,
    None,
}
impl StyledNode {
    // Return the specified value of a property if it exists, otherwise `None`.
    fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    // The value of the `display` property (defaults to inline).
    fn display(&self) -> Display {
        match self.value("display") {
            Some(Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}
fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // Create the root box.
    let mut root = LayoutBox::new(match style_node.display() {
        Block => BlockNode(style_node),
        Inline => InlineNode(style_node),
        DisplayNone => panic!("Root node has display: none."),
    });

    // Create the descendant boxes.
    for child in &style_node.children {
        match child.display() {
            Block => root.children.push(build_layout_tree(child)),
            Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            DisplayNone => {} // Skip nodes with `display: none;`
        }
    }
    return root;
}

impl LayoutBox {
    // Constructor function
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
        }
    }
    // ...
}

// Where a new inline child should go.
fn get_inline_container(&mut self) -> &mut LayoutBox {
    match self.box_type {
        InlineNode(_) | AnonymousBlock => self,
        BlockNode(_) => {
            // If we've just generated an anonymous block box, keep using it.
            // Otherwise, create a new one.
            match self.children.last() {
                Some(&LayoutBox {
                    box_type: AnonymousBlock,
                    ..
                }) => {}
                _ => self.children.push(LayoutBox::new(AnonymousBlock)),
            }
            self.children.last_mut().unwrap()
        }
    }
}

// The Layout Tree
