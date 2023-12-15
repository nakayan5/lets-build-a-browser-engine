# lets-build-a-browser-engine

[Let's build a browser engine!](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html)を読んで、ブラウザエンジンを作ってみる。

## Part1

## Part2

```mermaid
flowchart TD
    A[parse Function] -->|Creates Parser instance| B(Parser Instance)
    B --> C{parse_nodes}
    C -->|Loop over content| D[parse_node]
    D -->|Check next character| E{Is '<' ?}
    E -->|Yes| F[parse_element]
    E -->|No| G[parse_text]
    F --> H[parse_tag_name for opening tag]
    H --> I[parse_attributes]
    I --> J[Recursively call parse_nodes for children]
    J --> K[parse_tag_name for closing tag]
    K --> L[Return DOM element node]
    G --> M[Consume text until '<' and return DOM text node]
    C --> N[Return nodes array]
    N --> O{Is nodes length 1?}
    O -->|Yes| P[Return single node]
    O -->|No| Q[Create and return 'html' element with nodes]

    click A "javascript:void(0);" "Function to parse HTML document"
    click B "javascript:void(0);" "Parser instance with current position and input"
    click C "javascript:void(0);" "Function to parse multiple nodes"
    click D "javascript:void(0);" "Function to parse a single node"
    click E "javascript:void(0);" "Decision based on the next character"
    click F "javascript:void(0);" "Function to parse an element node"
    click G "javascript:void(0);" "Function to parse a text node"
    click H "javascript:void(0);" "Function to parse the tag name of an opening tag"
    click I "javascript:void(0);" "Function to parse attributes of a tag"
    click J "javascript:void(0);" "Recursive call to parse child nodes"
    click K "javascript:void(0);" "Function to parse the tag name of a closing tag"
    click L "javascript:void(0);" "Return the parsed element node"
    click M "javascript:void(0);" "Consume and return text until the next '<'"
    click N "javascript:void(0);" "Return an array of parsed nodes"
    click O "javascript:void(0);" "Decision based on the length of nodes array"
    click P "javascript:void(0);" "Return the single node if only one is present"
    click Q "javascript:void(0);" "Create and return a new 'html' element node"


```

```

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

実装

- [robinson](https://github.com/mbrubeck/robinson)

- https://github.com/qnighy/htstream
  - [HTML パーサーの設計・実装ノート (1) 字句解析](https://zenn.dev/qnighy/articles/0c9a49fd00069a)
  - [HTML パーサーの設計・実装ノート (2) 構文解析](https://zenn.dev/qnighy/articles/1a6ec268986cfd)

その他

- https://shikiyura.com/2022/08/install_the_multiple-runtime-versions_management_tool__asdf/
```
