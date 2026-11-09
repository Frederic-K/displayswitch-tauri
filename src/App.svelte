<script>
  // Fenêtre principale : état des écrans, Télétravail, Personnel, Actualiser et Configurer.
  // L'apparence commune (couleurs, tailles, boutons) vient de theme.css.
  import * as api from "./lib/api.js";
  import MonitorList from "./components/MonitorList.svelte";
  import ConfigureDialog from "./components/ConfigureDialog.svelte";

  let displays = $state([]);
  let displaysError = $state(null);
  // Résultat de la dernière action, information ou erreur.
  let message = $state("");
  // Proposition affichée dans la boîte « Configurer Télétravail » tant qu'elle est ouverte.
  let proposal = $state(null);
  // Une action est en cours : les boutons sont désactivés pour éviter un double clic.
  let busy = $state(false);

  async function refreshDisplays() {
    try {
      displays = await api.listDisplays();
      displaysError = null;
    } catch (error) {
      displaysError = error;
    }
  }

  // Lancer une action, afficher son message s'il y en a un, puis relire les écrans.
  async function runAction(action) {
    busy = true;
    try {
      const text = await action();
      if (text) message = text;
    } catch (error) {
      message = `Erreur : ${error}`;
    }
    await refreshDisplays();
    busy = false;
  }

  // Ouvrir la boîte de confirmation avec les rôles détectés dans Windows.
  function startConfiguration() {
    runAction(async () => {
      proposal = await api.detectConfiguration();
    });
  }

  function saveConfiguration() {
    const saved = proposal;
    proposal = null;
    runAction(() => api.saveConfiguration(saved));
  }

  refreshDisplays();
</script>

<main>
  <h1>Display Switch</h1>

  <MonitorList {displays} error={displaysError} />

  <div class="button-row">
    <button class="large" disabled={busy} onclick={() => runAction(api.telework)}>
      Télétravail
    </button>
    <button class="large" disabled={busy} onclick={() => runAction(api.personal)}>
      Personnel
    </button>
  </div>

  <!-- Ligne d'état : vide, elle garde sa hauteur pour que rien ne bouge. -->
  <p class="message">{message}</p>

  <div class="button-row">
    <button disabled={busy} onclick={() => runAction(refreshDisplays)}>Actualiser</button>
    <button disabled={busy} onclick={startConfiguration}>Configurer Télétravail…</button>
  </div>
</main>

{#if proposal}
  <ConfigureDialog {proposal} onsave={saveConfiguration} oncancel={() => (proposal = null)} />
{/if}

<style>
  /* Les blocs s'empilent de haut en bas, séparés de --space-3. */
  main {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4);
  }

  .message {
    min-height: 1.3em;
    text-align: center;
  }
</style>
