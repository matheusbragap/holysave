<script lang="ts">
  import AppShell from "$lib/components/AppShell.svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import StatusBanner from "$lib/components/StatusBanner.svelte";
</script>

<AppShell>
  <div slot="topbar">
    <PageHeader currentRoute="Configuracoes" showStats={false} showRefresh={false} />
  </div>

  <StatusBanner>
    Pagina de configuracoes (placeholder). Em breve voce podera ajustar conta, backups e
    preferencias.
  </StatusBanner>
</AppShell>
