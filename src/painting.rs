// https://limpet.net/mbrubeck/2014/11/05/toy-layout-engine-7-painting.html
use crate::css::Color;
use crate::css::Value;
use crate::layout::BoxType::AnonymousBlock;
use crate::layout::BoxType::BlockNode;
use crate::layout::BoxType::InlineNode;
use crate::layout::LayoutBox;
use crate::layout::Rect;
/// この記事では、ごく基本的なペイントコードを追加する。このコードはlayoutモジュールからボックスのツリーを受け取り、それらをピクセルの配列に変える。
/// この処理は "ラスタライズ "とも呼ばれる。
/// ブラウザはドキュメントの構造、各要素のスタイル、ページのジオメトリ、ペイント順序を認識し、ページをどのように描画するのでしょうか。この情報を画面上のピクセルに変換することを ラスタライズと呼びます
/// ブラウザは通常、SkiaやCairo、Direct2DなどのグラフィックスAPIやライブラリの助けを借りてラスタライズを実装します。
/// これらのAPIは、多角形、直線、曲線、グラデーション、テキストを描画する関数を提供しています。今のところ、矩形しか描けない独自のラスタライザを書こうと思う。
/// いずれはテキスト・レンダリングを実装したい。その時には、このおもちゃのペイント・コードを捨てて、「本物の」2Dグラフィックス・ライブラリに切り替えるかもしれない。
/// しかし、今のところ、私のブロック・レイアウト・アルゴリズムの出力を画像化するには矩形で十分だ。

type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand {
    SolidColor(Color, Rect),
    // insert more commands here
}

fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    // TODO: render text
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

///
fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

// CSS プロパティ `name` に指定された色、または指定されなかった場合は None を返す。
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    // MEMO：本家と異なる
    match &layout_box.box_type {
        &BlockNode(ref style) | &InlineNode(ref style) => match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None,
        },
        AnonymousBlock => None,
    }
}

///
fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return, // border-colorが指定されていない場合はベールアウトする。
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    // Right border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    // Top border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    // Bottom border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    // 真っ白なキャンバスを作る
    fn new(width: usize, height: usize) -> Canvas {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        Canvas {
            pixels: vec![white; width * height],
            width,
            height,
        }
    }

    fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            &DisplayCommand::SolidColor(color, rect) => {
                // Clip the rectangle to the canvas boundaries.
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        // TODO: alpha compositing with existing pixel
                        self.pixels[x + y * self.width] = color;
                    }
                }
            }
        }
    }
}

/// LayoutBoxes のツリーをピクセルの配列にペイントします。
pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
    for item in display_list {
        canvas.paint_item(&item);
    }
    canvas
}
