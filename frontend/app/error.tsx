"use client";

export default function GlobalError({ reset }: { reset: () => void }) {
  return (
    <main>
      <h1>問題が発生しました</h1>
      <p>時間をおいて、もう一度お試しください。</p>
      <button type="button" onClick={reset}>
        再試行する
      </button>
    </main>
  );
}
