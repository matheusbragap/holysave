<script lang="ts">
  /** Faixa curta (`default` | `error`) com conteúdo via `{@render children()}`. */

  import type { Snippet } from "svelte";

  let {
    tone = "default",
    children,
  }: {
    tone?: "default" | "error";
    children: Snippet;
  } = $props();
</script>

<p class={tone === "error" ? "status error" : "status"}>
  {@render children()}
</p>

<style>
  .status {
    max-width: 980px;
    margin: 0 auto;
    padding: 12px 16px;
    border-radius: var(--radius-md);
    background: var(--panel-strong);
    color: var(--muted);
    border: 1px solid var(--border);
  }

  .status.error {
    background: #ffe3e3;
    color: #7a2020;
    border-color: #f0b7b7;
  }
</style>
