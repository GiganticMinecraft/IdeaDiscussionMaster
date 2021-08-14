# IdeaDiscussionMaster
ギガンティック☆整地鯖公式Discordにて、アイデア会議を円滑に進行するためのDiscord Botです。

## Commands

// TODO: コマンドの引数などの詳細説明

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
