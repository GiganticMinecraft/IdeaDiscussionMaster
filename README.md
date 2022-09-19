# IdeaDiscussionMaster

ギガンティック☆整地鯖公式Discordにて、アイデア会議を円滑に進行するためのDiscord Botです。

## コマンド

|コマンド|説明|
|---|---|
|start|会議を開始するコマンド。Redmineと通信を行って、議事録に関連付けられているチケット番号をすべて取得し、次の議題を選択・提示する。|
|end|会議を終了するコマンド。会議結果を表示し、議事録チケットに結果を記載して、チケットと会議を終了する。|
|vote start|投票を開始するコマンド。投票を促すメッセージを投稿し、所定のリアクションをそのメッセージに付与する。なお、当該メッセにVCの過半数を超えるリアクションがつくと、自動で`vote end`コマンドと同等の処理を行う。|
|vote end|議題を終了するコマンド。指定されたステータスで議題チケットを更新・記録して、次の議題を選択・提示する。|
|agenda add|議題を追加するコマンド。指定されたチケットを議事録チケットに関連付け、現在の議題を再抽選、表示する。|
|create new_record|次回の議事録チケットを作成するコマンド。|
|create issue|SeichiAssistにIssueを追加するコマンド。|
|create thread|承認された議題につき個別の議論をするために各議題ごとにスレッドを作成するコマンド。|

### 議題ステータス

以下のアルファベットのみで指定できます。大文字・小文字や全角・半角は区別されます。

|アルファベット|日本語|
|---|---|
|Approved|承認|
|Declined|却下|

## 実行するには

* Dockerコンテナでの利用が想定されていますが、バイナリ単体での実行も可能です。
* `docker-compose.yml`やバイナリと同じディレクトリに`.env`と`key.pem`が必要です。
  * `.env`: 実行に必要な環境変数。詳細は[こちら](#環境変数)を参照してください。
  * `key.pem`: [GitHub App][2]の秘密鍵。

### 環境変数

|変数名|説明|
|---|---|
|DISCORD_TOKEN|DiscordBotのToken。[Discord Developer Portal][1]から入手。|
|DISCORD_APPLICATION_ID|DiscordBotのApplication ID。[Discord Developer Portal][1]から入手。|
|DISCORD_GUILD_ID|Botを作動させるサーバーのID。|
|REDMINE_API_KEY|RedmineのAPIキー。アイデア提案チケットと議事録チケットの追加・修正などを行うので、適切な権限設定が必要。|
|GH_APP_ID|[GitHub App][2]のID。Issueの作成をラベル付きで行うので、SeichiAssistにプッシュができるアカウントのものが必要。|

## License

[MIT License](./LICENSE)

### Some codes are licensed under the other one

See [this file](./crate-presentation/src/shared/command/README.md).

[1]: https://discord.com/developers/docs
[2]: https://docs.github.com/ja/developers/apps/building-github-apps/authenticating-with-github-apps
