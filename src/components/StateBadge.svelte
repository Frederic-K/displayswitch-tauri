<script>
  // Point d'état toujours accompagné de son mot : disque vert « Actif », anneau gris « Inactif ».
  let { active } = $props();
</script>

<span class="badge">
  <span class="dot" class:active></span>
  {active ? "Actif" : "Inactif"}
</span>

<style>
  .badge {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    font-size: var(--text-small);
    white-space: nowrap;
  }

  .dot {
    width: 8px;
    height: 8px;
    border: 1px solid var(--border-control);
    border-radius: 50%;
  }

  .dot.active {
    background: var(--success);
    border-color: var(--success);
  }
</style>
