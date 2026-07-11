# Future AI Considerations

AI is prohibited in the MVP. No external AI SDK, model, embedding, network call, or AI-labeled rule behavior will be added.

If separately approved later, boundaries may include `SearchProvider`, `SuggestionProvider`, `SummaryProvider`, and `SensitiveDataFilter`, with a deterministic local implementation as the baseline. A future design must compare local models with explicit opt-in cloud processing; show the exact context before sending; redact before transmission; exclude secrets; record requests without sensitive payloads; support deletion; bound cost and rate; and treat commands/logs as prompt-injection input. Interfaces are documented only and are not premature code abstractions.
