import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import "./app.css";

export default function App() {
  const toggleTheme = () => {
    document.body.classList.toggle("dark");
  };
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
              Change Theme
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
