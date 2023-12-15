// https://limpet.net/mbrubeck/2014/08/11/toy-layout-engine-2.html

use crate::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize,    // 現在の解析位置
    input: String, // 入力されたHTMLの文字列
}

impl Parser {
    // パーサが現在処理している文字を返す。posがinputの長さ以上の場合は空白を返す。
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // パーサの現在位置から始まる文字列が特定の文字列sで始まっているかどうかをチェックする。
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // 入力文字列がすべて消費されたかどうかを判断する。
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // パーサの現在位置にある文字を返し、その後pos（パーサの現在位置）を次の文字に進めます。
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices(); // 文字とそのバイト位置のペアを提供します。
        let (_, cur_char) = iter.next().unwrap(); // 現在の文字とその位置を取得
        let (next_pos, _) = iter.next().unwrap_or((1, ' ')); // 次の文字の位置を取得します。もし次の文字がなければ、デフォルトとして(1, ' ')（1文字分進める）が返されます。
        self.pos += next_pos;
        return cur_char;
    }

    // 指定された条件（test関数）がtrueを返す間、文字を消費し続けます。
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    // Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(CharExt::is_whitespace);
    }

    // HTMLタグの名前を解析します。
    // タグ名は英数字（'a'から'z'、'A'から'Z'、'0'から'9'）で構成されていると仮定しています。
    // <div>や<span>といったタグにおいて、"div"や"span"という文字列を抽出します。
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => true,
            _ => false,
        })

        // return "sample".to_owned();
    }

    // 単一のノード（要素またはテキスト）を解析します。
    // self.next_char()が'<'である場合、要素ノード（self.parse_element()）を解析し、そうでない場合はテキストノード（self.parse_text()）を解析します。
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    // テキストノードを解析します。
    // self.consume_whileを使用して、次の'<'までの文字を消費します。
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    // 要素ノード（開始タグ、内容、終了タグを含む）を解析します。
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        // DOMを返す
        return dom::elem(tag_name, attrs, children);
    }

    // 単一の属性（例：class="example"）を解析します。
    // 属性名と値のペア（例：("class", "example")）が返されます。
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // 属性値を解析します。
    // 最初に開始引用符をself.consume_charで消費し、次に引用符が再び現れるまでの間、文字を消費します。
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // 複数の属性を解析し、HashMapとして返します。
    // タグの終了（>）が見つかると、属性のHashMapを返します。
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    // 複数の兄弟ノード（隣接するノード）を解析します。
    // 空白をスキップし、現在位置がファイルの終わりか、閉じタグの開始（"</"）であるかを確認します。
    // 閉じタグに達するか、入力が終わるまで、self.parse_nodeを繰り返し呼び出して各ノードを解析し、ベクタに追加します。
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

// HTML文書全体を解析して、そのルート要素（DOMツリーの最上位のノード）を返す関数です。
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}
