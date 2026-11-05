import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";

const root = document.getElementById("app");

function showFrontendFailure(): void {
  if (!root) return;

  root.setAttribute("aria-busy", "false");
  root.innerHTML = `
    <main class="startup-state startup-state--error" role="alert">
      <h1>Life Planner</h1>
      <p>A interface não conseguiu iniciar.</p>
      <p>Feche e abra o aplicativo novamente. Se o problema continuar, procure o suporte do projeto.</p>
    </main>
  `;
}

try {
  if (!root) throw new Error("Elemento raiz #app não encontrado.");

  const app = createApp(App);
  app.config.errorHandler = (error, _instance, information) => {
    console.error(`[Life Planner][frontend-runtime] Falha ao iniciar a interface (${information}).`, error);
    showFrontendFailure();
  };
  app.mount(root);
  root.dataset.ready = "true";
  root.setAttribute("aria-busy", "false");
} catch (error) {
  console.error("[Life Planner][frontend-bootstrap] Não foi possível montar a interface.", error);
  showFrontendFailure();
}
