<script lang="ts">
  /** Placeholder de configurações; mesma casca visual que a biblioteca. */

  import AppShell from "$lib/components/layout/AppShell.svelte";
  import PageHeader from "$lib/components/layout/PageHeader.svelte";
  import StatusBanner from "$lib/components/ui/StatusBanner.svelte";
</script>

<AppShell>
  <div slot="topbar">
    <PageHeader currentRoute="Configuracoes" />
  </div>

  <StatusBanner>
    Pagina de configuracoes (placeholder). Em breve voce podera ajustar conta, backups e
    preferencias.
  </StatusBanner>
</AppShell>
