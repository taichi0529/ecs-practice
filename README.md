## ECS入門その1

## ゴール
超簡単なマイクロサービスを作って遊んでみる

## ECSってなに

- コンテナを管理するしくみ。
- 手作業だと大変でしょ。（何が？）
- タスク定義
- サービス

## とりあえず触ってみる

### 1. ECRへリポジトリをつくってみる

AWSのコンソールで作る。

### 2. ローカルでイメージを作成

nginxのイメージを作って動かしてみる。


### 3. ECRにログイン

```
aws ecr --profile xxxxx get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin xxxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com
```

### 4. ECRへpush

```
docker push xxxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com/nginx
```

### 5. ECRクラスタ作成
### 6. タスク定義作成
### 7. サービス作成



## オートスケーリングについて

タスクとインスタンスのオートスケーリング

実際に触ってみる（資料は後でつくります）


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


