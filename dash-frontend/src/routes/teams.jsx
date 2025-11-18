import "../styles/global.css";
import Nav from "./nav";

export default function teams() {
  return (
    <div style={{ padding: "20px", textAlign: "center" }}>
      <Nav />
      <h2>Teams Page</h2>
      <p>This is the teams page. Content coming soon!</p>
      <a href="/drivers">Go to Home</a>
    </div>
  );
}
