<script lang="ts">
  /** Árvore recursiva só para vizualização (nós vindos dos mocks). */

  import FolderSubtree from "./FolderTree.svelte";
  import type { MockTreeNode } from "./mock-save-tree";

  let { node, depth = 0 }: { node: MockTreeNode; depth?: number } = $props();

  let isFolder = $derived(Boolean(node.children?.length));
</script>

<div class="tree-node" style:padding-left="{depth === 0 ? 0 : 18}px">
  {#if depth > 0}
    <span class="corner" aria-hidden="true">{isFolder ? "◢" : "·"}</span>
  {/if}
  <span class="label" class:folder={isFolder} class:root={depth === 0}>{node.name}</span>
</div>
{#if node.children}
  {#each node.children as child}
    <FolderSubtree node={child} depth={depth + 1} />
  {/each}
{/if}

<style>
  .tree-node {
    display: flex;
    align-items: baseline;
    gap: 8px;
    font-family: ui-monospace, "Cascadia Code", monospace;
    font-size: 0.78rem;
    line-height: 1.65;
    color: var(--ink);
  }

  .corner {
    flex-shrink: 0;
    color: var(--muted);
    font-size: 0.65rem;
    opacity: 0.85;
  }

  .label {
    word-break: break-all;
    color: var(--muted);
  }

  .label.folder {
    color: var(--accent-strong);
    font-weight: 600;
  }

  .label.root {
    font-size: 0.82rem;
    color: var(--ink);
    font-weight: 600;
  }
</style>
