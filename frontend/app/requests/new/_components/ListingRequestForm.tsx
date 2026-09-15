"use client";

import { useState, useTransition } from "react";
import { createListingRequestAction } from "../actions";

export function ListingRequestForm() {
  const [sourceUrl, setSourceUrl] = useState("");
  const [message, setMessage] = useState("");
  const [isError, setIsError] = useState(false);
  const [isPending, startTransition] = useTransition();

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    startTransition(async () => {
      const result = await createListingRequestAction(sourceUrl);
      setIsError(!result.ok);
      setMessage(result.message);
      if (result.ok && result.message.includes("受け付けました"))
        setSourceUrl("");
    });
  }

  return (
    <form onSubmit={handleSubmit}>
      <div className="form-field">
        <label htmlFor="source-url">URL</label>
        <input
          id="source-url"
          name="sourceUrl"
          type="url"
          inputMode="url"
          placeholder="https://example.com/..."
          required
          value={sourceUrl}
          onChange={(event) => setSourceUrl(event.target.value)}
        />
      </div>
      <button type="submit" disabled={isPending}>
        {isPending ? "送信中..." : "送信する"}
      </button>
      {message && (
        <output className={isError ? "form-error" : "form-message"}>
          {message}
        </output>
      )}
    </form>
  );
}
