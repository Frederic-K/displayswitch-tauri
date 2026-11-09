<script>
  // Une ligne d'écran : nom et détail à gauche, état à droite.
  import StateBadge from "./StateBadge.svelte";

  let { display } = $props();

  // Nom à afficher, même si Windows n'en fournit pas.
  let name = $derived(display.name || "Moniteur sans nom");

  // « 3840 × 2160 — Principal », ou rien d'utile si l'écran est inactif.
  let detail = $derived.by(() => {
    if (!display.active || !display.resolution) return "Résolution indisponible";
    const [width, height] = display.resolution;
    const role = display.primary ? "Principal" : "Secondaire";
    return `${width} × ${height} — ${role}`;
  });
</script>

<div class="row">
  <div>
    <p class="name">{name}</p>
    <p>{detail}</p>
  </div>
  <StateBadge active={display.active} />
</div>

<style>
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
  }

  /* Trait entre deux écrans. */
  .row + :global(.row) {
    margin-top: var(--space-2);
    padding-top: var(--space-2);
    border-top: 1px solid var(--border);
  }

  .name {
    color: var(--ink);
    font-size: var(--text-monitor-name);
  }
</style>
