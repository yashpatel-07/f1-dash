import { A } from "@solidjs/router";
import "../styles/global.css";

export default function Nav() {
  const NAV_ITEMS = [
    { name: "Drivers", path: "/drivers" },
    { name: "Races", path: "/races" },
    { name: "Teams", path: "/teams" },
  ];
  return (
    <nav class="nav-bar">
      <div class="nav-links">
        {NAV_ITEMS.map((item) => (
          <A href={item.path} class="nav-link" activeClass="active">
            {item.name}
          </A>
        ))}
      </div>
    </nav>
  );
}
