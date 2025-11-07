import { For, createSignal, on, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "../styles/global.css";

type DriverStanding = {
  driver: string;
  position: number;
  points: string;
};

export default function Drivers() {
  const [Standings, setStandings] = createSignal<DriverStanding[]>([]);

  onMount(async () => {
    try {
      const result = await invoke<DriverStanding[]>("get_drivers_standings");
      setStandings(result);
    } catch (error) {
      console.error("Error fetching driver standings:", error);
    }
  });

  return (
    <div class="drivers-page">
      <h1>Driver Standings</h1>
      <table class="drivers-table">
        <thead>
          <tr>
            <th>Driver</th>
            <th>Position</th>
            <th>Points</th>
          </tr>
        </thead>
        <tbody>
          <For each={Standings()}>
            {(d) => (
              <tr>
                <td>{d.driver}</td>
                <td>{d.position}</td>
                <td>{d.points}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}
