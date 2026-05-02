<script lang="ts">
  export let tone: "default" | "error" = "default";
</script>

<p class={tone === "error" ? "status error" : "status"}>
  <slot />
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
