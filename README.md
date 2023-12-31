# lets-build-a-browser-engine

[Let's build a browser engine!](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html)を読んで、ブラウザエンジンを作ってみる。

<p style="background-color: white;">
  <image src="https://limpet.net/mbrubeck/images/2014/pipeline.svg" />
</p>

## Command

```bash
$ cargo build

$ ./target/debug/lets-build-a-browser-engine --html examples/test.html --css examples/test.css
```

## Part1 & Part2

HTML の文字列を受け取って、DOM ツリーを構築する。

```mermaid
stateDiagram-v2
state nodes_length <<choice>>


[html] --> parse
parse --> parse_nodes

parse_nodes --> consume_whitespace
consume_whitespace --> parse_node

parse_node --> parse_element
parse_node --> parse_text

parse_element　--> Node
parse_text --> Node

Node --> nodes_length
nodes_length --> root_element: if nodes_length == 1
nodes_length --> dom_elem : if nodes_length != 1

```

## part3

タグ名、ID、'.'で始まる任意の数のクラス名をサポートする。

```mermaid
stateDiagram-v2
  state end <<choice>>

  classDef parent display:flex,align-items:center,justify-content:center,background-color:#fff,border-radius:5px,padding:10px

  [css] --> parse
  parse --> parse_rules:::parent
  parse_rules --> end

  end --> Stylesheet: if end true
  end --> parse_rule: if end false

  parse_rule --> parse_rules

  parse_rule --> parse_selectors
  parse_rule --> parse_declarations
  parse_selectors --> parse_rule
  parse_declarations --> parse_rule
```

## part4

DOM ツリー内の各ノードはスタイル ツリー内に正確に 1 つのノードを持ちます。
スタイル ツリーを構築する最初のステップは、セレクタのマッチングです。
単純なセレクタが要素にマッチするかどうかは、要素そのものを見ればわかります。
複合セレクタをマッチさせるには、DOM ツリーを走査して要素の兄弟や親などを調べる必要があります。

## part5 & part6

スタイル ツリーを 2 次元空間の矩形に変換するレイアウト モジュール。
layout モジュールの入力はパート 4 のスタイルツリーで、出力はさらに別のツリーのレイアウトツリーです。

## References

記事

- [「Let's build a browser engine!」を読んで Rust で簡易レンダリングエンジンを作った](https://dackdive.hateblo.jp/entry/2021/02/23/113522)

  - [Inside look at modern web browser (part 3)](https://developer.chrome.com/blog/inside-browser-part3/)
  - https://github.com/zaki-yama/rust-toy-browser-engine

- [Populating the page: how browsers work](https://www.linkedin.com/pulse/understanding-browser-rendering-critical-path-divyansh-singh/)

- [How browser rendering works — behind the scenes](https://blog.logrocket.com/how-browser-rendering-works-behind-scenes/)

- [Populating the page: how browsers work](https://developer.mozilla.org/en-US/docs/Web/Performance/How_browsers_work)

- [Deno で簡易レンダリングエンジンを作ってみた](https://zenn.dev/ryo_kawamata/articles/920baf76bfdf2e)

- [How web browsers work - parsing the HTML (part 3, with illustrations)](https://dev.to/arikaturika/how-web-browsers-work-parsing-the-html-part-3-with-illustrations-45fi)

- [ちいさな Web ブラウザを作ってみよう](https://browserbook.shift-js.info/)
  - [Webブラウザセキュリティ Webアプリケーションの安全性を支える仕組みを整理する](https://www.amazon.co.jp/Web%E3%83%96%E3%83%A9%E3%82%A6%E3%82%B6%E3%82%BB%E3%82%AD%E3%83%A5%E3%83%AA%E3%83%86%E3%82%A3-Web%E3%82%A2%E3%83%97%E3%83%AA%E3%82%B1%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3%E3%81%AE%E5%AE%89%E5%85%A8%E6%80%A7%E3%82%92%E6%94%AF%E3%81%88%E3%82%8B%E4%BB%95%E7%B5%84%E3%81%BF%E3%82%92%E6%95%B4%E7%90%86%E3%81%99%E3%82%8B-%E7%B1%B3%E5%86%85%E8%B2%B4%E5%BF%97/dp/4908686106/ref=sr_1_1?crid=1PTQJV3GP886B&keywords=web%E3%83%96%E3%83%A9%E3%82%A6%E3%82%B6%E3%82%BB%E3%82%AD%E3%83%A5%E3%83%AA%E3%83%86%E3%82%A3&qid=1704267627&sprefix=web%E3%83%96%E3%83%A9%E3%82%A6%E3%82%B6%2Caps%2C190&sr=8-1)

実装

- [robinson](https://github.com/mbrubeck/robinson)

- https://github.com/qnighy/htstream
  - [HTML パーサーの設計・実装ノート (1) 字句解析](https://zenn.dev/qnighy/articles/0c9a49fd00069a)
  - [HTML パーサーの設計・実装ノート (2) 構文解析](https://zenn.dev/qnighy/articles/1a6ec268986cfd)

その他

- https://shikiyura.com/2022/08/install_the_multiple-runtime-versions_management_tool__asdf/
- [Eliminate content repaints with the new Layers panel in Chrome](https://blog.logrocket.com/eliminate-content-repaints-with-the-new-layers-panel-in-chrome-e2c306d4d752/?gi=cd6271834cea)
