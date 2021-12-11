# IdeaDiscussionMaster

ギガンティック☆整地鯖公式Discordにて、アイデア会議を円滑に進行するためのDiscord Botです。

## コマンド

|コマンド|説明|エイリアス|使用例|
|---|---|---|---|
|start_discussion|会議を開始するコマンド。Redmineと通信を行って、議事録に関連付けられているチケット番号をすべて取得し、次の議題を選択・提示する。|sid, sdi|`\sid [議事録のチケット番号]`, `\sid 100`|
|start_votes|投票を開始するコマンド。投票を促すメッセージを投稿し、所定のリアクションをそのメッセージに付与する。なお、当該メッセにVCの過半数を超えるリアクションがつくと、自動で`end_discussion`コマンドと同等の処理を行う。|svo|`\svo`|
|end_votes|投票を終了するコマンド。指定されたステータスで議題チケットを更新・記録して、次の議題を選択・提示する。|evo|`\evo` [[議題ステータス]](#議題ステータス), `\evo 承認`, `\evo approved`, `\evo app`|
|add_agenda|議題を追加するコマンド。指定されたチケットを議事録チケットに関連付け、現在の議題を再抽選、表示する。|ada, aag|`\ada [チケット番号]`, `\sid 1000`|
|end_discussion|会議を終了するコマンド。会議結果を表示し、議事録チケットに結果を記載して、チケットと会議を終了する。|eid, edi|`\eid`|
|show_agendas|すべての議題の進行状況を表示するコマンド。|sha|`\sha`|
|create_next_record|次回の議事録チケットを作成するコマンド。|cnr|`\cnr`|
|add_github_issue|SeichiAssistにIssueを追加するコマンド。|agi, ghissue, ghi|`\ghissue [議事録のチケット番号] [Issueを作成するチケットの番号群（半角スペース区切り）]`, `\ghissue 12345 6789 1023`|

### 議題ステータス

以下のいずれかで指定できます。英字の大文字・小文字は区別されません。全角・半角は区別され、前者では認識しません。

|フル|ショート|日本語|
|---|---|---|
|approved|app|承認|
|declined|dec|却下|

## 環境設定ファイルについて（.env）

|変数名|説明|
|---|---|
|DISCORD_TOKEN|DiscordBotのToken。[Discord Developer Portal][1]から入手。|
|REDMINE_KEY|RedmineのAPIキー。アイデア提案チケットと議事録チケットの追加・修正などを行うので、適切な権限設定が必要。|
|EXECUTABLE_ROLE_ID|DiscordロールのロールID。ここで指定したIDのロールをもつユーザーだけが本Botのコマンドを実行できる。|
|GITHUB_KEY|GitHubのPersonalAccessToken。Issueの作成をラベル付きで行うので、SeichiAssistにプッシュができるアカウントのものが必要。|

## License

[MIT License](./LICENSE)

[1]: https://discord.com/developers/docs
