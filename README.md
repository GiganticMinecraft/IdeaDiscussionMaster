# IdeaDiscussionMaster

ギガンティック☆整地鯖公式Discordにて、アイデア会議を円滑に進行するためのDiscord Botです。

## コマンド

|コマンド|説明|使用例|
|---|---|---|
|start|会議を開始するコマンド。Redmineと通信を行って、議事録に関連付けられているチケット番号をすべて取得し、次の議題を選択・提示する。|`/start [議事録のチケット番号]`, `/start 100`|
|end|会議を終了するコマンド。会議結果を表示し、議事録チケットに結果を記載して、チケットと会議を終了する。|`/end`|
|vote start|投票を開始するコマンド。投票を促すメッセージを投稿し、所定のリアクションをそのメッセージに付与する。なお、当該メッセにVCの過半数を超えるリアクションがつくと、自動で`vote end`コマンドと同等の処理を行う。|`/vote start`|
|vote end|議題を終了するコマンド。指定されたステータスで議題チケットを更新・記録して、次の議題を選択・提示する。|`/vote end` [[議題ステータス]](#議題ステータス), `/vote end Approved`|
|agenda add|議題を追加するコマンド。指定されたチケットを議事録チケットに関連付け、現在の議題を再抽選、表示する。|`/agenda add [チケット番号]`, `/agenda add 1000`|
|agenda list|すべての議題の進行状況を表示するコマンド。|`/agenda list`|
|create new_record|次回の議事録チケットを作成するコマンド。|`/create new_record`|
|create issue|SeichiAssistにIssueを追加するコマンド。|`/create issue [議事録のチケット番号] [Issueを作成するチケットの番号群（コンマ区切り）]`, `/create issue 12345 6789,1023`|

### 議題ステータス

以下のアルファベットで指定できます。大文字・小文字は区別されません。全角・半角は区別され、前者では認識しません。

|アルファベット|日本語|
|---|---|
|Approved|承認|
|Declined|却下|

## 環境設定ファイルについて（.env）

|変数名|説明|
|---|---|
|DISCORD_TOKEN|DiscordBotのToken。[Discord Developer Portal][1]から入手。|
|DISCORD_APPLICATION_ID|DiscordBotのApplication ID。[Discord Developer Portal][1]から入手。|
|REDMINE_KEY|RedmineのAPIキー。アイデア提案チケットと議事録チケットの追加・修正などを行うので、適切な権限設定が必要。|
|EXECUTABLE_ROLE_ID|DiscordロールのロールID。ここで指定したIDのロールをもつユーザーだけが本Botのコマンドを実行できる。|
|GH_APP_ID|GitHub AppのID。Issueの作成をラベル付きで行うので、SeichiAssistにプッシュができるアカウントのものが必要。|
|GH_APP_RSA_KEY_PATH|GitHub AppのPrivateSSHキーへのファイルパス。コンテナ側のマウントされるパスなので注意。省略した場合は、`/key.pem`が読み込まれる。|

## License

[MIT License](./LICENSE)

### Some codes are licensed under the other one

See [this file](./src/utils/commands/README.md).

[1]: https://discord.com/developers/docs
