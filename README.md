# Snake - AariaToys
AariaToysの第1作、[ヘビゲーム](https://github.com/marcusbuffett/bevy_snake)から派生したBevy製ゲームです。
![Snake](https://1.bp.blogspot.com/-Ys7HZCD3P4o/X5o5t-HbZkI/AAAAAAAABDI/szooc0ANLvgWHsJLSm7jQ2HYN8aJvPCGACLcBGAsYHQ/s1032/snake.jpg)
## Game rules
  通常のヘビゲームとは、以下の点で違いがあります。
- 自分の尾や、画面枠に当たってもゲームオーバーにはなりません。  
- キーを押している間だけ移動します。移動処理は下記「SPEED」のタイミングで行われるため、設定によっては操作感が変わります。通常のヘビゲームは自動で移動し、操作は方向を指定するだけです。  
- 180度真後ろに加え、斜め移動もできます。一般的なヘビゲームでは方向転換は90度のみです。  
- Enterキーでいつでもリセット(ヘビと餌を初期化)できます。  
- スコアやステージクリアや競争相手などはなく、ただひたすら餌を捕り、伸びていくシンプルなゲームです。  
- 様々な要素を変更できるため、プレイ感は設定次第で大きく変わります。子供がブロックで遊ぶように、能動的/創造的にゲームプレイを楽しめます。

## Customize
[config.txt](https://github.com/Aariar/snake/blob/main/config.txt)にて、ゲーム設定を自由に調整することができます(ゲーム再起動で反映)。  
半角:の後の値(数値かbool値)部分のみ書き換えます。簡易な処理をしているため、「:値(空白不可)」以外はあまりいじらないでください。  
具体的な処理は[config_load](https://github.com/Aariar/snake/blob/main/src/main.rs)で確認できます。  

- width_num ： 横マスの数を指定(ヘビはこのマス単位で移動します)。
- height_num ： 縦マスの数を指定。
- snake_speed ： ヘビ速度をms(1/1000秒)単位で指定します。このタイミングでヘビの位置は更新されます。
- food_pop ： 餌の出現頻度をms単位で設定します。
- tail_shrink ： trueにすると伸びたヘビの尾はどんどん縮まっていきます(true以外の文字列は全てfalse扱い)。
- win_width ： ゲームウィンドウの幅をpixel単位で指定します。
- win_height ： ゲームウィンドウの幅をpixel単位で指定します。

## How to Play ?
AariaToysシリーズはデジタル世界のおもちゃのブロックのように、自由に遊び方(ルール)を設定し遊べるゲームです。  
そこに決まったルールはなく、ただただデジタルの世界で「遊ぶ」ことを目的としています。  
もちろん、上記設定項目以外にも、コードでスコアを追加したり、ヘビに画像を当てはめられるよう改良するのもご自由に。  
snake.exeから起動する場合は、同じフォルダにconfig.txtがある必要があるので、ご注意ください。  
新しい改良バージョンが出来上がったらご一報いただければこちらからリンクさせていただきますし、あるいは当バージョンに組み込ませていただくかも知れません。  
自由に作り、自由に設定し、自由に遊び、自由に共有する……[Bevy](https://bevyengine.org/)と共に

## Data
- compile time ： 7m 44s (cargo run --release)  
- exe file ： 10.4MB
