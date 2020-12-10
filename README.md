## ECS入門その1

## ゴール
超簡単なマイクロサービスを作って遊んでみる。
分かりやすさを優先しているので正確性は欠けていると思います。（そもそも自分の理解が甘い可能性も）

## ECSってなに

- コンテナを管理するしくみ。
- サーバーが10台あったら全部手作業だと大変というか無理！
- クラスタ、タスク、サービスの３つをまずは覚えよう。
- k8sと違って抽象化されていないので、ELBとかセキュリティグループなかのAWSの機能をそのまま使うけれど、難しいことしなければいいようにやってくれる。


### タスク
コンテナの集合。例えばnginxとphpとコードのコンテナとか。
一つのアプリケーションを動かしている集まり。

### サービス
タスクをどうやって動かすかを管理してくれる。
例えばタスクをデプロイするときは１台ずつとか全部まとめてとか、インスタンスの数とかそういうの。
別にサービスなくてもタスクを主導で動かすことはできる。

### クラスタ
タスクとかサービスの集まり。

## 静的なWebサイトをECSで管理してみる。
まずは簡単な練習として静的なファイルを
nginxのイメージを作って動かしてみる。

### 1. ECRへリポジトリをつくってみる

AWSのコンソールでECR上にリポジトリを作る。名前は`ecs-handson/nginx`で。
そうするとこのイメージのURIは下記の様になります。

```
xxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com/ecs-handson/nginx
```

後々面倒くさいので環境変数にECRのリポジトリを登録しておきます。

```bash
ECR_REPOSITORY=xxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com
```

### 2. ローカルでイメージを作成
用意してあるDockerfileを使ってnginxのイメージを作成する。
中身はnginxのイメージを持ってきてindex.htmlをイメージの中にコピーしているだけ。

```bash
cd nginx
docker build -t ${ECR_REPOSITORY}/ecs-handson/nginx:1 ./
```

### 3. ECRにログイン

aws cliを使ってログイン用のパスワードを取得します。
それをパイプでdockerコマンドへ渡しています。

```
aws ecr --profile xxxxx get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin ${ECR_REPOSITORY}
```

### 4. ECRへpush

```
docker push ${ECR_REPOSITORY}/ecs-handson/nginx:1
```

### 5. 必要なIAMロールの作成

ECSのインスタンス用とタスクを実行するときに使うロール（ECSのインスタンスに付いているロールでいい気がするんだけど何で別なんだろ）を作っておきます。
（タスク用のロールもそのうち必要になるけれどとりあえず置いておく）
管理コンソールから自動で作成されるんだけれど何やっているのか分かっていた方がいいので手動で作って見ます。

#### ECSインスタンス用ロール

ロール名は何でもいいのだがここでは`ecsHandsOnInstanceRole`とつけて、

- AmazonEC2ContainerServiceforEC2Role
- AmazonSSMManagedInstanceCore

の二つのポリシーをつける。
 `AmazonEC2ContainerServiceforEC2Role`はEC2インスタンス上で動くECSのエージェントのためのポリシーです。
ECRからpullする、CloudWatchにログを書いたり、ECSのAPIを叩いたり等など。
[こちら](https://docs.aws.amazon.com/ja_jp/AmazonECS/latest/developerguide/instance_IAM_role.html) を参照の事。
`AmazonSSMManagedInstanceCore`はセッションマネージャを使用して管理コンソールからインスタンスへログインするためにつけてあります

#### タスク実行用のロール
おもにFargate用？インスタンスロールでダメな理由がそれくらいしか思いつかないです。
[ドキュメント](https://docs.aws.amazon.com/ja_jp/AmazonECS/latest/developerguide/task_execution_IAM_role.html)にはEC2からもこちらのロールを使ってECRへアクセスしていると書いてあります。
ロール名は何でもいいのだがここでは`ecsHandsOnTaskExecutionRole`とつけて、

-  AmazonECSTaskExecutionRolePolicy

のポリシーをつけて下さい。ECRからpullする、CloudWatchにログを書くだけのポリシーです。インスタンスロールとかぶってる。
  

### 6. ECRクラスタ作成

ECSの`Clusters`のページで

1. `Create Cluster`のボタンを押して
1. `EC2 Linux + Networking`を選択して`Next step`
1. 下記の項目だけ入力
    - `Cluster name`をとりあえず`nginx`にする（なんでもいい）
    - `EC2 instance type` は `t3.micro`
    - `Container instance IAM role`は 5で作成した`ecsHandsOnInstanceRole`を選択
1. `Create`ボタンを押す
  
これだけでクラスターが出来てしまいます。裏でCloudFormationが動いてVPCなんかを作っているので確認してみて下さい。


### 6. タスク定義作成

ECSの`Task Definitions`のページで

1. `Create new Task Definitions`のボタンを押して
1. `EC2`を選択して`Next step`
1. 下記の項目だけ入力
    - `Task Definition Name`をとりあえず`nginx-task`にする（なんでもいい）
    - `Task execution role`を先ほどつくった`ecsHandsOnTaskExecutionRole`にする。
    - `Add Container`ボタンをおして
         - `Container name`をnginx
         - `Image`をECRにプッシュしたイメージ名。`xxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com/ecs-handson/nginx:1`
         - `Memory Limits`を128
1. `Create`を押す

これでできあがりです。


### 7. サービス作成

### 8. index.htmlを更新してみる。
index.htmlの中身を変更してイメージを作成する。

```bash
cd nginx
docker build -t xxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com/ecs-handson/nginx:2 ./
```

## CI/CDの実装

上記のnginxのコンテナをCodeBuild上で作る様にします。
それをECS上にデプロイするようにします。

CodeCommitへpush => CodeBuildでdockerイメージを作成してECRへpush => CodeDeployでECSのタスク定義を更新

という流れになります。

### 1. buildspec.ymlの作成

codePipelineはソースファイルを参照できる状態で`buildspec.yml`というファイルに書いてあるスクリプトを実行します。
実行する環境は色々選べます。

`buildspec.yml`がデフォルトだけれどもこのファイル名は指定出来るので今回は`buildspec_nginx.yml`で作ります。

### 2. gitリポジトリの作成
当然、githubとも連携できるけれど今回のハンズオンで出来るだけトラブルなく行うためにAWSのgitリポジトリのCodeCommitを使います。
管理画面上で `ecs-handson.nginx` という名前でgitリポジトリを作成して下さい。

### 3. CodePipelineの設定
CodePipelineを使用してCodeCommit, CodeBuild, CodeDeployを連携させます。

### 4. 更新されるかの確認。

`nginx/index.html`を更新してコミットしてpushしてみる。
上手く行けばECSのタスクが更新されるはず。

## オートスケーリング

ぶっちゃけオートスケーリングで適切にスケーリングさせるのは難しい・・・・・。
CPUに負荷をかけてオートスケーリングさせるの面倒くさいので

オートスケーリングさせるのは基本的にはEC2インスタンス

## デプロイ

- REPLICA
- DAEMON
- Capacity Providers
- FARGATE
- ターゲットグループとの連動

実際に触ってみる（資料は後でつくります）


## CI/CD

## k8sとの対比

## blue greenとローリングアップデート

# マイクロサービスを作って遊んでみる

- 認証・認可
- 商品一覧
- ショップ


