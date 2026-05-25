<script lang="ts">
  import FolderPicker from '$lib/FolderPicker.svelte';
  import RuleForm from '$lib/RuleForm.svelte';
  import ResultsList from '$lib/ResultsList.svelte';
  import ActionBar from '$lib/ActionBar.svelte';
  import { app } from '$lib/appState.svelte';
</script>

<main class="app">
  <header>
    <h1>replaceMd</h1>
    <p class="sub">Markdown 批量文本替换 · 预览后再执行 · 自动备份可还原</p>
  </header>

  <div class="card"><FolderPicker /></div>
  <div class="card"><RuleForm /></div>

  <div class="card">
    <ActionBar />
    {#if app.busy}
      <p class="busy">处理中…</p>
    {/if}
    <ResultsList />
  </div>
</main>

<style>
  :global(:root) {
    --bg: #f5f6f8;
    --surface: #ffffff;
    --border: #e3e6ea;
    --ink-1: #1c2024;
    --ink-2: #555c66;
    --ink-3: #9aa1ab;
    --accent: #3b6cf6;
    --accent-soft: #e7eeff;
    --add: #1a7f45;
    --add-soft: #e6f5ec;
    --del: #b4262a;
    --del-soft: #fbe9e9;
    font-family: 'Inter', -apple-system, 'Segoe UI', Roboto, system-ui, sans-serif;
    color: var(--ink-1);
  }

  :global(body) {
    margin: 0;
    background: var(--bg);
  }

  :global(.btn) {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--ink-1);
    border-radius: 8px;
    padding: 0.5rem 1rem;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
  }
  :global(.btn:hover:not(:disabled)) {
    border-color: var(--accent);
  }
  :global(.btn.primary) {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }
  :global(.btn.primary:hover:not(:disabled)) {
    filter: brightness(1.05);
  }
  :global(.btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .app {
    max-width: 860px;
    margin: 0 auto;
    padding: 1.5rem 1.25rem 3rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  header {
    margin-bottom: 0.25rem;
  }
  h1 {
    margin: 0;
    font-size: 1.5rem;
    letter-spacing: -0.01em;
  }
  .sub {
    margin: 0.25rem 0 0;
    color: var(--ink-2);
    font-size: 0.85rem;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 1rem 1.1rem;
  }
  .busy {
    margin: 0.6rem 0 0;
    color: var(--accent);
    font-size: 0.85rem;
  }
</style>
