"use client";

import { useState } from "react";

export const TryButton = () => {
  const [tried, setTried] = useState(false);

  return (
    <button type="button" onClick={() => setTried(true)}>
      {tried ? "試してみます！" : "試してみる"}
    </button>
  );
};
