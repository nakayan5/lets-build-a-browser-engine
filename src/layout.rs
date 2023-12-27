// https://limpet.net/mbrubeck/2014/09/08/toy-layout-engine-5-boxes.html
// https://limpet.net/mbrubeck/2014/09/17/toy-layout-engine-6-block.html
// @see https://developer.chrome.com/blog/inside-browser-part3?hl=ja

use crate::css::Unit::Px;
use crate::css::Value;
use crate::layout::BoxType::AnonymousBlock;
use crate::layout::BoxType::BlockNode;
use crate::layout::BoxType::InlineNode;
use crate::layout::Value::Keyword;
use crate::layout::Value::Length;
use crate::style::Display;
use crate::style::StyledNode;

use std::default::Default;

// CSS box model. All sizes are in px.
#[derive(Clone, Copy, Default, Debug)]
pub struct Dimensions {
    /// Position of the content area relative to the document origin:
    pub content: Rect,
    // Surrounding edges:
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[derive(Clone, Copy, Default, Debug)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
#[derive(Clone, Copy, Default, Debug)]
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
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
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

// impl StyledNode {
//     // Return the specified value of a property if it exists, otherwise `None`.
//     fn value(&self, name: &str) -> Option<Value> {
//         self.specified_values.get(name).map(|v| v.clone())
//     }

//     // The value of the `display` property (defaults to inline).
//     fn display(&self) -> Display {
//         match self.value("display") {
//             Some(Keyword(s)) => match &*s {
//                 "block" => Display::Block,
//                 "none" => Display::None,
//                 _ => Display::Inline,
//             },
//             _ => Display::Inline,
//         }
//     }
// }

fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // Create the root box.
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BlockNode(style_node),
        Display::Inline => InlineNode(style_node),
        Display::None => panic!("Root node has display: none."),
    });

    // Create the descendant boxes.
    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            Display::None => {} // Don't lay out nodes with `display: none;`
        }
    }
    root
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BlockNode(node) | InlineNode(node) => node,
            AnonymousBlock => panic!("Anonymous block box has no style node"),
        }
    }
}

impl<'a> LayoutBox<'a> {
    // Lay out a box and its descendants.
    // 幅を計算するときにツリーをトップダウンに走査して、親の幅がわかってから子要素をレイアウトし、高さを計算するときにボトムアップに走査して、親の高さを子要素の後で計算する必要があります。
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BlockNode(_) => self.layout_block(containing_block),
            InlineNode(_) => {}  // TODO
            AnonymousBlock => {} // TODO
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        // 子の幅は親の幅に依存することがあるので、次のように計算する必要がある。
        // 子ボックスをレイアウトする前に、このボックスの幅を計算する必要があります。
        self.calculate_block_width(containing_block);

        // コンテナ内のボックスの位置を決める。
        self.calculate_block_position(containing_block);

        // このボックスの子を再帰的に並べる。
        self.layout_block_children();

        // 親の高さは子の高さに依存することがあるので、`calculate_height`は子がレイアウトされた後に呼ばれなければならない。
        self.calculate_block_height();
    }

    // これはlookupと呼ばれるヘルパー関数を使用し、一連の値を順番に試します。
    // 最初のプロパティが設定されていなければ、2番目のプロパティを試します。
    // それも設定されていなければ、与えられたデフォルト値を返します。これは、省略記法のプロパティと初期値の不完全な（しかし単純な）実装です。
    // ex) margin_left = style["margin-left"] || style["margin"] || zero;
    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        // `width` has initial value `auto`.
        let auto = Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        // margin, border, and padding have initial value 0.
        let zero = Length(0.0, Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        // プロパティが'auto'に設定されている場合は0を返すので、合計には影響しません。
        let total = sum([
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| v.to_px()));

        // widthがautoでなく、totalがコンテナより広い場合、autoのマージンは0として扱う。
        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = Length(0.0, Px);
            }
            if margin_right == auto {
                margin_right = Length(0.0, Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            // 値がオーバーコンストレインドの場合は、margin_rightを計算する。
            (false, false, false) => {
                margin_right = Length(margin_right.to_px() + underflow, Px);
            }

            // ちょうど1つのサイズが自動である場合、その使用される値は等号から導かれる。
            (false, false, true) => {
                margin_right = Length(underflow, Px);
            }
            (false, true, false) => {
                margin_left = Length(underflow, Px);
            }

            // widthがautoに設定されている場合、その他のautoの値は0になる。
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Length(0.0, Px);
                }
                if margin_right == auto {
                    margin_right = Length(0.0, Px);
                }

                if underflow >= 0.0 {
                    // Expand width to fill the underflow.
                    width = Length(underflow, Px);
                } else {
                    // Width can't be negative. Adjust the right margin instead.
                    width = Length(0.0, Px);
                    margin_right = Length(margin_right.to_px() + underflow, Px);
                }
            }

            // margin-leftとmargin-rightがともにautoの場合、使用される値は等しくなる。
            (false, true, true) => {
                margin_left = Length(underflow / 2.0, Px);
                margin_right = Length(underflow / 2.0, Px);
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    // margin/padding/borderスタイルの再作成を検索し、これらを含むブロックの寸法と一緒に使用して、このブロックのページ上の位置を決定します。
    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        // margin, border, and padding have initial value 0.
        let zero = Length(0.0, Px);

        // If margin-top or margin-bottom is `auto`, the used value is zero.
        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        // Position the box below all the previous boxes in the container.
        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    // 子ボックスをループしながら、コンテンツの高さの合計を記録しています。これは、次の子の垂直方向の位置を見つけるための位置決めコード（上記）で使用されます。
    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // 各子コンテンツが前のコンテンツの下にレイアウトされるように、高さを追跡する。
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    /// デフォルトでは、ボックスの高さは中身の高さに等しい。しかし、'height' プロパティに明示的な長さが設定されている場合は、代わりにそれを使用します
    fn calculate_block_height(&mut self) {
        // 高さが明示的な長さに設定されている場合は、その長さを使用する。
        // そうでない場合は、`layout_block_children`で設定した値を保持する。
        if let Some(Length(h, Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }

    // Where a new inline child should go.
    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
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
}

/// * this does not implement margin collapsing
impl Dimensions {
    /// コンテンツ領域とそのパディングでカバーされる領域
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    /// コンテンツ領域にパディングとボーダーを加えた領域
    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    /// コンテンツ領域にパディング、ボーダー、マージンを加えた領域
    pub fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

impl Rect {
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

fn sum<I>(iter: I) -> f32
where
    I: Iterator<Item = f32>,
{
    iter.fold(0., |a, b| a + b)
}
