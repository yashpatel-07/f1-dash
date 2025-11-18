import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { createSignal, on, onMount, Suspense } from "solid-js";
import "./app.css";

export default function App() {
  let [theme, setTheme] = createSignal("dark");
  const toggleTheme = () => {
    const newTheme = theme() === "light" ? "dark" : "light";
    setTheme(newTheme);
    document.body.classList.toggle("dark", newTheme === "dark");
  };

  onMount(() => {
    if (theme() === "dark") {
      document.body.classList.add("dark");
    } else {
      document.body.classList.remove("dark");
    }
  });

  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <header
            style={{
              padding: "12px",
              display: "flex",
              "justify-content": "space-between",
            }}
          >
            <h1>F1 Dashboard</h1>
            <button class="btn-f1" onClick={toggleTheme}>
              {theme() === "light" ? "Dark" : "Light"}
            </button>
          </header>
          <Suspense>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
