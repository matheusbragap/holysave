<a class="settings" href="/configuracoes" title="Configuracoes" aria-label="Configuracoes">
  <svg viewBox="0 0 24 24" aria-hidden="true">
    <path
      d="M19.4 13a7.8 7.8 0 0 0 .1-1 7.8 7.8 0 0 0-.1-1l2-1.6-2-3.4-2.4 1a7.7 7.7 0 0 0-1.7-1l-.4-2.6H9.1l-.4 2.6c-.6.2-1.2.6-1.7 1l-2.4-1-2 3.4 2 1.6a7.8 7.8 0 0 0-.1 1 7.8 7.8 0 0 0 .1 1l-2 1.6 2 3.4 2.4-1c.5.4 1.1.8 1.7 1l.4 2.6h5.8l.4-2.6c.6-.2 1.2-.6 1.7-1l2.4 1 2-3.4-2-1.6zM12 15.5A3.5 3.5 0 1 1 12 8.5a3.5 3.5 0 0 1 0 7z"
    />
  </svg>
</a>

<style>
  .settings {
    position: fixed;
    right: 20px;
    bottom: 20px;
    width: 54px;
    height: 54px;
    border-radius: 16px;
    border: 1px solid rgba(224, 122, 63, 0.4);
    background: linear-gradient(135deg, #fff4da 0%, #ffe0c4 100%);
    box-shadow: 0 16px 30px rgba(27, 16, 4, 0.2);
    display: grid;
    place-items: center;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    z-index: 4;
    text-decoration: none;
  }

  .settings svg {
    width: 24px;
    height: 24px;
    fill: #7a3a14;
  }

  .settings:hover {
    transform: translateY(-2px);
    box-shadow: 0 20px 34px rgba(27, 16, 4, 0.26);
  }

  @media (max-width: 720px) {
    .settings {
      right: 16px;
      bottom: 86px;
    }
  }
</style>
