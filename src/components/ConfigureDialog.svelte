<script>
  // Boîte modale « Configurer Télétravail » : Enregistrer ou Annuler.
  // Échap ou un clic hors de la boîte valent Annuler.
  import { onMount } from "svelte";

  let { proposal, onsave, oncancel } = $props();
  let dialog;

  onMount(() => dialog.showModal());

  // Échap : on ferme nous-mêmes via oncancel, pour que l'état Svelte reste la seule source.
  function handleCancel(event) {
    event.preventDefault();
    oncancel();
  }

  // Le contenu remplit toute la boîte : un clic sur la boîte elle-même tombe sur le fond.
  function handleBackdropClick(event) {
    if (event.target === dialog) oncancel();
  }
</script>

<dialog bind:this={dialog} oncancel={handleCancel} onclick={handleBackdropClick}>
  <div class="content">
    <h2>Configurer Télétravail</h2>

    <div>
      <p>Écran à conserver : {proposal.primary_monitor}</p>
      <p>Écran à désactiver : {proposal.secondary_monitor}</p>
    </div>

    <div>
      <p>Ces réglages seront utilisés par le bouton Télétravail.</p>
      <p>Aucun changement d'affichage ne sera appliqué.</p>
    </div>

    <div class="button-row">
      <button onclick={onsave}>Enregistrer</button>
      <button onclick={oncancel}>Annuler</button>
    </div>
  </div>
</dialog>

<style>
  dialog {
    width: 340px;
    max-width: calc(100vw - 2 * var(--space-4));
    padding: 0;
    background: var(--surface);
    color: inherit;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  }

  dialog::backdrop {
    background: rgba(0, 0, 0, 0.25);
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-3);
  }

  .button-row {
    margin-top: var(--space-1);
  }
</style>
