# IdeaDiscussionMaster
ギガンティック☆整地鯖公式Discordにて、アイデア会議を円滑に進行するためのDiscord Botです。

## Commands

|コマンド|説明|エイリアス|使用例|
|---|---|---|---|
|start_discussion|会議を開始するコマンド。Redmineと通信を行って、議事録に関連付けられているチケット番号をすべて取得し、次の議題を選択・提示する。|sid|`\sid [議事録のチケット番号]`, `\sid 100`|
|start_votes|投票を開始するコマンド。投票を促すメッセージを投稿し、所定のリアクションをそのメッセージに付与する。なお、当該メッセにVCの過半数を超えるリアクションがつくと、自動で`end_discussion`コマンドと同等の処理を行う。|svo|`\svo`|
|end_votes|投票を終了するコマンド。指定されたステータスで議題チケットを更新・記録して、次の議題を選択・提示する。|evo|`\evo [議題ステータス(*)]`, `\evo 承認`, `\evo approved`, `\evo app`|
|end_discussion|会議を終了するコマンド。会議結果を表示し、議事録チケットに結果を記載して、チケットと会議を終了します。|eid|`\eid`|

### 議題ステータス

以下のいずれかで指定できます。英字の大文字・小文字は区別されませんが、全角・半角は区別され、前者では認識しません。

|フル|ショート|日本語|
|---|---|---|
|approved|app|承認|
|declined|dec|却下|

## About .env

|変数名|説明|
|---|---|
|IDEA_DISCUSSION_MASTER_DISCORD_TOKEN|DiscordBotのToken。[Discord Developer Portal](https://discord.com/developers/docs)から入手。|
|IDEA_DISCUSSION_MASTER_REDMINE_KEY|RedmineのAPIキー。アイデア提案チケットと議事録チケットの追加・修正などを行うので、適切な権限設定が必要|
|IS_DEBUG|「true」と設定しておくと、コマンドが実行された際にコンソールに通知を行う。それ以外を設定するもしくは何も設定を行わないと、当該通知は行われない。|
|EXECUTABLE_ROLE_ID|DiscordロールのロールID。ここで指定したIDのロールをもつユーザーだけが本Botのコマンドを実行できる。|

## docker

// TODO: docker

## License

[MIT License](./LICENSE)
