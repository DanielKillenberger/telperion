# Native-image transport verification

No model test call. Local `grok models` lists grok-4.6 as supported/default. CLI parser accepts --no-memory/--no-subagents/--no-plan/--no-auto-update. Explicit allowed read_file is then disallowed, with deny rules for execution/read/write/search/web/MCP; empty --tools is NOT used because it means no restriction.

- https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/src/headless/cli.rs : .json prompt-file reads ACP content blocks, preserves text block rather than flattening images.
- https://raw.githubusercontent.com/agentclientprotocol/agent-client-protocol/main/agent-client-protocol-schema/src/v1/content.rs : ImageContent data/mime_type with camelCase serde => data/mimeType; base64 bytes.
- https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-config-types/src/memory.rs : resolve_settings false CLI/env opt-out disables legacy andv2 memory. GROK_CONFIG memory override was not used because source allowlist excludes it.
- https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/src/headless.rs : JSON emitter keeps structuredOutput, usage/modelUsage alongside text/stopReason.
- Local user-guide14-headless-mode: total_tokens counts uncached/cache-read/cache-create/output; reasoning subset is not added again. Single grok-4.6 modelUsage/modelCalls1 required; unknown totals retain reservation.

Fresh scratch, neutral system override, no memory/plan/subagents/web. Same exact user prompt/schema/five image bytes; provider/system scaffolding is not identical to Codex/Claude. Temporary base64 input removed with scratch; never printed or committed. Raw response local; compact answer/usage/model pinned in journal.
