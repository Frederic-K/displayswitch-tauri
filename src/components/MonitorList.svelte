<script>
  // Bloc des écrans : une ligne par écran, séparées par un trait.
  import MonitorRow from "./MonitorRow.svelte";

  let { displays, error } = $props();
</script>

<section>
  {#if error}
    <p>Lecture des écrans impossible : {error}</p>
  {:else if displays.length === 0}
    <p>Aucun écran disponible.</p>
  {:else}
    {#each displays as display (display.pathIndex)}
      <MonitorRow {display} />
    {/each}
  {/if}
</section>

<style>
  section {
    padding: var(--space-2) var(--space-3);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
  }
</style>
