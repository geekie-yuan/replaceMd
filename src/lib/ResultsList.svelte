<script lang="ts">
  import { app, setAll, toggleFile } from '$lib/appState.svelte';

  let expanded = $state<Record<string, boolean>>({});
  function toggleExpand(path: string) {
    expanded = { ...expanded, [path]: !expanded[path] };
  }
</script>

{#if app.preview}
  {#if app.preview.files.length === 0}
    <p class="empty">没有命中任何文件。</p>
  {:else}
    <div class="bulk">
      <span class="summary">命中 {app.preview.matched_files} 个文件 / 共 {app.preview.total_matches} 处</span>
      <span class="spacer"></span>
      <button class="link" onclick={() => setAll(true)}>全选</button>
      <button class="link" onclick={() => setAll(false)}>全不选</button>
    </div>

    <ul class="files">
      {#each app.preview.files as f (f.path)}
        <li>
          <div class="file-head">
            <label class="file-check">
              <input
                type="checkbox"
                checked={!!app.selected[f.path]}
                onchange={() => toggleFile(f.path)}
              />
              <span class="fname" title={f.path}>{f.path}</span>
            </label>
            <span class="count">{f.match_count} 处</span>
            <button class="link" onclick={() => toggleExpand(f.path)}>
              {expanded[f.path] ? '收起' : '查看'}
            </button>
          </div>

          {#if expanded[f.path]}
            <div class="snippets">
              {#each f.snippets as s}
                <div class="snippet">
                  <span class="lno">L{s.line_no}</span>
                  <div class="lines">
                    <div class="before">- {s.before}</div>
                    <div class="after">+ {s.after}</div>
                  </div>
                </div>
              {/each}
              {#if f.match_count > f.snippets.length}
                <div class="more">… 还有更多命中未一一展示</div>
              {/if}
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}

  {#if app.preview.errors.length}
    <details class="errors">
      <summary>{app.preview.errors.length} 个文件读取失败（已跳过）</summary>
      <ul>
        {#each app.preview.errors as e}<li>{e}</li>{/each}
      </ul>
    </details>
  {/if}
{/if}

<style>
  .empty {
    color: var(--ink-3);
    text-align: center;
    padding: 1.5rem 0;
  }
  .bulk {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin-bottom: 0.5rem;
    font-size: 0.85rem;
  }
  .summary {
    color: var(--ink-2);
    font-weight: 600;
  }
  .spacer {
    flex: 1;
  }
  .files {
    list-style: none;
    margin: 0;
    padding: 0;
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }
  .files li + li {
    border-top: 1px solid var(--border);
  }
  .file-head {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.5rem 0.7rem;
  }
  .file-check {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    min-width: 0;
    cursor: pointer;
  }
  .fname {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.82rem;
    color: var(--ink-1);
  }
  .count {
    flex: none;
    font-size: 0.78rem;
    color: var(--ink-2);
    background: var(--accent-soft);
    border-radius: 999px;
    padding: 0.1rem 0.55rem;
  }
  .link {
    background: none;
    border: none;
    color: var(--accent);
    cursor: pointer;
    font-size: 0.82rem;
    padding: 0.1rem 0.3rem;
  }
  .link:hover {
    text-decoration: underline;
  }
  .snippets {
    padding: 0.3rem 0.7rem 0.7rem 2rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }
  .snippet {
    display: flex;
    gap: 0.6rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.8rem;
  }
  .lno {
    flex: none;
    color: var(--ink-3);
  }
  .lines {
    min-width: 0;
    flex: 1;
  }
  .before,
  .after {
    white-space: pre-wrap;
    word-break: break-word;
    border-radius: 4px;
    padding: 0.05rem 0.3rem;
  }
  .before {
    background: var(--del-soft);
    color: var(--del);
  }
  .after {
    background: var(--add-soft);
    color: var(--add);
  }
  .more {
    color: var(--ink-3);
    font-size: 0.78rem;
  }
  .errors {
    margin-top: 0.75rem;
    font-size: 0.82rem;
    color: var(--del);
  }
  .errors ul {
    margin: 0.3rem 0 0;
    padding-left: 1.1rem;
  }
</style>
