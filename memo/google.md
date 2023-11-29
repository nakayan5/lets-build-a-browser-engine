# Inside look at modern web browser (part 3)

## レンダラー・プロセスはウェブ・コンテンツを処理する
レンダラー プロセスは、タブ内で発生するすべての処理を担当します。レンダラー プロセスでは、メイン スレッドがユーザーに送信するコードのほとんどを処理します。Web ワーカーやサービス ワーカーを使用すると、JavaScript の一部がワーカー スレッドで処理されることがあります。コンポジタースレッドとラスタースレッドもレンダラー プロセス内で実行され、ページを効率的かつスムーズにレンダリングします。
レンダラー プロセスの主な仕事は、HTML、CSS、JavaScript をユーザーが操作できる Web ページに変換することです。

<p align="center">
  <img src="https://wd.imgix.net/image/T4FyVKpzu4WKF1kBNvXepbi08t52/uIqf0QQZxF6mHPDWFEjz.png?auto=format&w=800" />
</p>

## Parsing

### Construction of a DOM
レンダラー・プロセスがナビゲーションのコミット・メッセージを受信し、HTMLデータの受信を開始すると、メイン・スレッドはテキスト文字列（HTML）の解析を開始し、それをDOM（Document Object Model）に変換する。
DOM はブラウザのページ内部表現であると同時に、ウェブ開発者が JavaScript を介してやり取りできるデータ構造と API です。
HTMLドキュメントをDOMに解析することは、HTML標準によって定義されています。ブラウザにHTMLを送信しても、決してエラーを投げないことにお気づきかもしれません。例えば、</p>タグが閉じられていないのは、有効なHTMLです。こんにちは！<b>I'm <i>Chrome</b>!</i>（bタグはiタグの前に閉じられています）のような誤ったマークアップは、Hi!<b>私は<i>クローム</i></b><i>!</i>です。これは、HTMLの仕様がこれらのエラーを優雅に扱うように設計されているからです。これらのことがどのように行われているのか気になる方は、HTML仕様の「[パーサーにおけるエラー処理と奇妙なケースの紹介](https://html.spec.whatwg.org/multipage/parsing.html#an-introduction-to-error-handling-and-strange-cases-in-the-parser)」のセクションをお読みください。

### Subresource loading
ウェブサイトは通常、画像、CSS、JavaScriptなどの外部リソースを使用します。これらのファイルはネットワークやキャッシュからロードする必要がある。メインスレッドは、DOMを構築するためのパース中にそれらを見つけると、ひとつひとつリクエストすることができるが、スピードアップのために、「プリロード・スキャナー」が並行して実行される。HTMLドキュメント内に<img>や<link>のようなものがある場合、プリロード・スキャナーはHTMLパーサーが生成したトークンを覗き見し、ブラウザ・プロセス内のネットワーク・スレッドにリクエストを送る。

<p align="center">
  <img src="https://wd.imgix.net/image/T4FyVKpzu4WKF1kBNvXepbi08t52/qmuN5aduuEit6SZfwVOi.png?auto=format&w=800" />
</p>

### JavaScript can block the parsing
HTML パーサーは <script> タグを見つけると、HTML ドキュメントの解析を一時停止し、JavaScript コードをロード、解析、実行しなければなりません。なぜかというと、JavaScriptはdocument.write()のようなものを使ってドキュメントの形を変えることができ、DOM構造全体を変えてしまうからです（[HTML仕様の構文解析モデルの概要](https://html.spec.whatwg.org/multipage/parsing.html#overview-of-the-parsing-model)に、すばらしい図があります）。これが、HTMLパーサーがHTML文書の解析を再開する前にJavaScriptの実行を待たなければならない理由です。JavaScriptの実行で何が起こっているのか興味がある方は、V8チームがこれに関する講演やブログ投稿を行っています。

## Style calculation
DOMがあるだけでは、ページがどのように見えるかを知るには十分ではない。メイン スレッドは CSS を解析し、各 DOM ノードに対して計算されたスタイルを決定します。これは、CSS セレクタに基づいて各要素にどのようなスタイルが適用されるかについての情報です。この情報はDevToolsのcomputedセクションで見ることができます。
CSSを指定しなくても、各DOMノードは計算されたスタイルを持ちます。< h1 >タグは< h2 >タグよりも大きく表示され、各要素にマージンが定義されます。これは、ブラウザにデフォルトのスタイルシートがあるためです。ChromeのデフォルトCSSがどのようなものか知りたい方は、こちらのソースコードをご覧ください。

<p align="center">
  <img src="https://wd.imgix.net/image/T4FyVKpzu4WKF1kBNvXepbi08t52/hGqtsAuYpEYX4emJd5Jw.png?auto=format&w=800" />
</p>

## Layout
レイアウトは要素のジオメトリを見つけるための処理です。メイン・スレッドはDOMと計算されたスタイルを走査し、x y座標やバウンディング・ボックスのサイズなどの情報を持つレイアウト・ツリーを作成します。レイアウトツリーはDOMツリーと似た構造かもしれませんが、ページ上に表示されているものに関連する情報しか含まれていません。display: noneが適用されている場合、その要素はレイアウト・ツリーの一部ではありません（ただし、visibility: hiddenが適用されている要素はレイアウト・ツリーに含まれます）。同様に、p::before{content: "Hi!"}のような内容を持つ擬似クラスが適用された場合、それがDOMになくてもレイアウトツリーに含まれます。

## Paint
DOM、スタイル、レイアウトがあるだけでは、ページをレンダリングするにはまだ不十分です。例えば、ある絵画を再現しようとしているとしよう。要素の大きさ、形、位置はわかっていても、どのような順番で描くかを判断しなければなりません。
例えば、特定の要素にz-indexが設定されている場合があります。その場合、HTMLに記述された要素の順番通りに描画すると、間違ったレンダリングになってしまいます。
このペイント・ステップでは、メイン・スレッドがレイアウト・ツリーを走査してペイント・レコードを作成する。ペイント・レコードとは、「まず背景、次にテキスト、そして矩形」というように、ペイントのプロセスを記録したものだ。JavaScriptを使って<canvas>要素に絵を描いたことがある人なら、このプロセスはおなじみかもしれない。

### Updating rendering pipeline is costly
