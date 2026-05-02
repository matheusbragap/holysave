<script lang="ts">
  import { page } from "$app/stores";
</script>

<nav class="rail" aria-label="Menu">
  <div class="rail-main">
    <a
      class="rail-item"
      class:is-active={$page.url.pathname === "/"}
      href="/"
      title="Jogos"
      aria-label="Jogos"
      aria-current={$page.url.pathname === "/" ? "page" : undefined}
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <rect x="3" y="4" width="18" height="6" rx="2" />
        <rect x="3" y="14" width="18" height="6" rx="2" />
      </svg>
    </a>
  </div>
  <div class="rail-bottom">
    <a
      class="rail-item rail-item--minimal"
      class:is-active={$page.url.pathname.startsWith("/configuracoes")}
      href="/configuracoes"
      title="Configuracoes"
      aria-label="Configuracoes"
      aria-current={$page.url.pathname.startsWith("/configuracoes") ? "page" : undefined}
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path
          d="M19.4 13a7.8 7.8 0 0 0 .1-1 7.8 7.8 0 0 0-.1-1l2-1.6-2-3.4-2.4 1a7.7 7.7 0 0 0-1.7-1l-.4-2.6H9.1l-.4 2.6c-.6.2-1.2.6-1.7 1l-2.4-1-2 3.4 2 1.6a7.8 7.8 0 0 0-.1 1 7.8 7.8 0 0 0 .1 1l-2 1.6 2 3.4 2.4-1c.5.4 1.1.8 1.7 1l.4 2.6h5.8l.4-2.6c.6-.2 1.2-.6 1.7-1l2.4 1 2-3.4-2-1.6zM12 15.5A3.5 3.5 0 1 1 12 8.5a3.5 3.5 0 0 1 0 7z"
        />
      </svg>
    </a>
  </div>
</nav>

<style>
  .rail {
    position: relative;
    align-self: stretch;
    height: 100%;
    width: 84px;
    padding: 20px 10px;
    background: var(--rail);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .rail-main {
    display: grid;
    gap: 12px;
  }

  .rail-bottom {
    margin-top: auto;
    padding-top: 8px;
  }

  .rail-item {
    border: none;
    background: transparent;
    border-radius: 14px;
    padding: 10px;
    display: grid;
    place-items: center;
    cursor: pointer;
    transition: transform 0.2s ease, background 0.2s ease;
    text-decoration: none;
    color: inherit;
  }

  .rail-item svg {
    width: 22px;
    height: 22px;
    fill: #5f5644;
  }

  .rail-item.is-active {
    background: rgba(255, 255, 255, 0.7);
    box-shadow: inset 0 0 0 1px rgba(200, 150, 90, 0.35);
  }

  .rail-item--minimal {
    opacity: 0.8;
  }

  .rail-item:hover {
    transform: translateY(-1px);
    background: rgba(255, 255, 255, 0.7);
  }

  @media (max-width: 720px) {
    .rail {
      width: 64px;
      padding: 16px 8px;
    }
  }
</style>
