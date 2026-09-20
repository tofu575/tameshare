# 9/26 運用準備と公開判断

## 実施したこと

- GitHub Actionsのworkflowを追加した。ComposeのテストDBを用意し、Backend fmt・Clippy・workspaceテスト、テストDBへのseed、Frontend `npm audit`・型検査・lint・build・Playwright E2Eを順に実行する設定。
- README、Backend・Frontendの手順、環境変数例、ローカル起動・migration・seed・匿名署名鍵・公開時確認の運用手順を更新した。
- FrontendのPostCSSとUndiciを修正版に固定し、Orvalを更新した。`npm ci` と `npm audit` は成功し、監査結果は0件。
- `cargo fetch --locked` が新規checkoutで動くよう、Backendの `Cargo.lock` をgitignore対象から外した。

## 確認方法と結果

- CIのYAMLはRubyのパーサで読み込み、job定義を確認した。GitHub Actions上での実行は未確認。
- `.git`、`.envrc`、`node_modules`、`target`、`.next` を含めない別コピーで `make setup` を実行し、環境ファイル作成、依存関係導入、FrontendとBackendの `make check` が成功した。この確認後、元のコピーの `Cargo.lock` がgitignore対象だと判明したため、追跡対象へ変更した。実際のDB・Backend・Frontend起動とブラウザ操作は元の作業コピーで別途確認済み。別コピーでの全起動はポート競合を避けるため未実施。
- `git diff --check` 成功。ローカルの通常DBに対する主要フローは9/21〜9/25に確認済み。

## 公開判断

今回は公開切替を進めない。公開先と秘密鍵の保管方法が指定されておらず、公開環境の主要フローと権限判定は未確認。旧方式との後方互換は不要と確認済み。公開先が決まったら、切替直前に操作、公開先、確認結果、残る問題を提示して判断を求める。

## 残る問題（重要度順）

1. 新しいCI workflowはGitHub上で未実行。pushまたはPR後の結果確認は後日実施する。
2. 別コピーでのDB・Backend・Frontend全起動は未確認。第三者による手順のみの通し起動は後日実施する。
3. 公開環境の構築と実環境での主要フロー・権限判定は後日実施する。

## 次に最初に行うこと

9/27は重大問題だけを最終確認し、主要フローとテストを再検査したうえで、公開見送りの理由と次の一手をまとめる。
