# main algorithm

```mermaid
sequenceDiagram
    participant cl as クライアント
    participant sv as クイズサーバー
    participant bl as ルームブロードキャスト

    cl ->>+ sv : ルーム加入req
    sv -->>- cl : 要求res
    cl ->>+ sv : クイズ開始req
    sv ->>+ bl : クイズ開始をブロードキャスト
    bl -->>- sv : 受理res
    sv ->>+ bl : 1. 問題を配信
    bl ->> bl : 制限時間を設定
    bl -->>- sv : 受理res
    cl ->>+ sv : 解答権req
    sv ->>+ bl : 解答権をロック
    bl -->>- sv : ロック受理res
    sv -->>- cl : 解答権受理res
    cl ->>+ sv : 解答send
    sv -->>- cl : 解答を判定
    sv ->>+ bl : ロック解除
    bl -->>- sv : 解除
    alt 正解 
        Note over cl : goto 1.
    else 誤答
        cl ->> sv : 解答を要求
    end
```

## クイズセクション 状態遷移

```mermaid
stateDiagram
    [*] --> 準備完了
    準備完了 --> クイズ配信
    クイズ配信 --> 解答権リク待ち
    解答権リク待ち --> 解答待ち
    解答待ち --> 解答
    解答 --> クイズ配信
    解答 --> 成績発表
    成績発表 --> [*]
    
```

# Client to Server

| コード            | 戻り値                 | 実行結果                     | 補足                                                                   |
| :---------------- | :--------------------- | :--------------------------- | ---------------------------------------------------------------------- |
| /join {room_name} | /join_ok or /join_err  | ルーム加入を試みる           |                                                                        |
| /start            | /quiz_started          | クイズセクションを開始する   | ルームに加入している場合のみ実行可能．これはルーム加入者全員に送られる |
| /ans_req          | /ans_ok or /ans_err    | 解答権を得る                 | 先着１名                                                               |
| /answer {解答}    | /correct or /incorrect | 解答に対しての正誤を判定する |                                                                        |

# Server to Client
| コード                          | 戻り値 | 実行結果                   | 補足                       |
| :------------------------------ | :----- | :------------------------- | -------------------------- |
| /quiz_started                   | なし   | クイズセクション開始を合図 | ルーム加入者全員に送られる |
| /question {limit_time} {問題文} | なし   |                            |                            |
| /ans_lock                       | なし   | 解答権をロックする         |                            |
| /ans_unlock                     | なし   | 解答権をアンロックする     |                            |
| /others_correct_answer {id} {answer}                    | なし   | 他ユーザの正解解答を通知する     |                            |
| /others_incorrect_answer {id} {answer}                    | なし   | 他ユーザの誤答解答を通知する     |                            |
| /users {user_json}              | なし   | ユーザ情報を配信         | 定期的に配信．型定義は以下 |

User {
    id,
    name,
    score
}