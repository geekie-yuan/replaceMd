<script lang="ts">
  import { app, chooseFolder } from '$lib/appState.svelte';
</script>

<section class="picker">
  <button class="btn" onclick={chooseFolder} disabled={app.busy}>选择目录…</button>
  <span class="path" class:empty={!app.folder} title={app.folder}>
    {app.folder || '尚未选择目录'}
  </span>
</section>

<style>
  .picker {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .path {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.85rem;
    color: var(--ink-2);
  }
  .path.empty {
    color: var(--ink-3);
    font-style: italic;
  }
</style>
