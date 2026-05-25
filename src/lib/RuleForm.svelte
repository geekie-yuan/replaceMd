<script lang="ts">
  import { app, runPreview } from '$lib/appState.svelte';

  const canPreview = $derived(!!app.folder && app.find.length > 0 && !app.busy);
</script>

<section class="rule">
  <label class="field">
    <span>查找</span>
    <textarea bind:value={app.find} rows="2" placeholder="要查找的文本"></textarea>
  </label>

  <label class="field">
    <span>替换为</span>
    <textarea bind:value={app.replace} rows="2" placeholder="替换成的文本（留空表示删除匹配内容）"></textarea>
  </label>

  <div class="toggles">
    <label><input type="checkbox" bind:checked={app.isRegex} /> 正则表达式</label>
    <label><input type="checkbox" bind:checked={app.caseSensitive} /> 区分大小写</label>
    <label><input type="checkbox" bind:checked={app.recursive} /> 包含子目录</label>
  </div>

  <button class="btn primary" onclick={runPreview} disabled={!canPreview}>扫描 / 预览</button>
</section>

<style>
  .rule {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }
  .field > span {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--ink-2);
  }
  textarea {
    resize: vertical;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.9rem;
    padding: 0.55rem 0.7rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--surface);
    color: var(--ink-1);
  }
  textarea:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-soft);
  }
  .toggles {
    display: flex;
    flex-wrap: wrap;
    gap: 1.2rem;
    font-size: 0.85rem;
    color: var(--ink-2);
  }
  .toggles label {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    cursor: pointer;
  }
</style>
