'use client';

import { useState } from 'react';

export const TryButton = () => {
  const [tried, setTried] = useState(false);

  return (
    <button onClick= {() => setTried(true)}>
      { tried? '試した！': '試してみる' }
    </button>
  );
}
