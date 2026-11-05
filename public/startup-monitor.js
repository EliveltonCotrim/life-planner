(() => {
  const showFailure = () => {
    const root = document.getElementById("app");
    if (!root || root.dataset.ready === "true") return;

    root.setAttribute("aria-busy", "false");
    root.innerHTML = `
      <main class="startup-state startup-state--error" role="alert">
        <h1>Life Planner</h1>
        <p>A interface não conseguiu iniciar.</p>
        <p>Feche e abra o aplicativo novamente. Se o problema continuar, procure o suporte do projeto.</p>
      </main>
    `;
  };

  window.addEventListener("error", (event) => {
    if (event.target instanceof HTMLScriptElement || event.error) {
      console.error("[Life Planner][frontend-bootstrap] Falha ao carregar ou iniciar a interface.", event.error ?? event.message);
      showFailure();
    }
  }, true);

  window.addEventListener("unhandledrejection", (event) => {
    console.error("[Life Planner][frontend-bootstrap] Falha assíncrona durante a inicialização da interface.", event.reason);
    showFailure();
  });
})();
