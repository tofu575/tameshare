# 9/27 最終確認

## 完了したこと

- 9/20の構成調査と9/21のローカル主要操作確認、9/22の問題整理。
- 匿名ユーザーの公開APIなりすまし更新を再現し、Backend発行の署名付きtokenで拒否できるよう修正した。本人・別ユーザー・認証なし・不正tokenと、拒否時のDB不変を確認した。
- 同一ユーザー・同一Practiceの一意制約と原子的な保存処理を追加した。並行POSTは同じExperience IDを返し、DBでは1件だけだった。
- Frontend主要フローのPlaywright検査、Backendの回帰テスト、CI workflow、起動・運用資料を追加した。
- 最終確認でBackend `make check`、Frontend `make check`、`npm audit`（0件）、`make e2e-test`（1件成功）、`make smoke`（4項目成功）を実行した。通常DBにはPractice 32件、Experience 11件、重複組0件。確認用Experienceは削除していない。
- 新規checkoutの `cargo fetch --locked` に必要な `backend/Cargo.lock` を追跡対象に変更した。

## 未完了のことと影響

1. GitHub ActionsのworkflowはローカルでYAML構文を確認しただけで、GitHub上では未実行。pushまたはPR後の結果確認は後日行う。
2. 別コピーで `make setup` と両サービスの `make check` は成功したが、DB・Backend・Frontendの全起動は未確認。第三者による手順のみの通し起動は後日行う。
3. 公開環境の構築と実環境の主要フロー・権限判定は未実施。公開先も未指定で、後日対応する。

旧方式のUUID Cookieとの後方互換は不要と確認済み。移行作業は行わない。

## 公開判断

公開は見送る。公開先と署名鍵の保管方法が決まっておらず、実環境も未確認。公開先URLはない。

## 次に着手する一手

公開作業に戻る際、まずCIをGitHub上で動かして結果を確認する。その後、第三者による通し起動と公開環境での主要フロー・権限判定を検証する。
