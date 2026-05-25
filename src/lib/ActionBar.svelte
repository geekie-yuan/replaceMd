<script lang="ts">
  import { app, runApply, runRestore, selectedPaths } from '$lib/appState.svelte';

  const selectedCount = $derived(selectedPaths().length);
  const canApply = $derived(!!app.preview && selectedCount > 0 && !app.busy);
</script>

<section class="actions">
  <button class="btn primary" onclick={runApply} disabled={!canApply}>
    执行替换{selectedCount ? `（${selectedCount} 个文件）` : ''}
  </button>
  <button class="btn" onclick={runRestore} disabled={app.busy || !app.folder}>
    从最近备份还原
  </button>
</section>

{#if app.status}<p class="msg ok">{app.status}</p>{/if}
{#if app.error}<p class="msg err">{app.error}</p>{/if}

<style>
  .actions {
    display: flex;
    gap: 0.75rem;
  }
  .msg {
    margin: 0.6rem 0 0;
    font-size: 0.85rem;
    white-space: pre-wrap;
    word-break: break-word;
    padding: 0.5rem 0.7rem;
    border-radius: 8px;
  }
  .msg.ok {
    background: var(--add-soft);
    color: var(--add);
  }
  .msg.err {
    background: var(--del-soft);
    color: var(--del);
  }
</style>
