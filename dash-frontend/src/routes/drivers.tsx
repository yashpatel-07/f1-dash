import { For, createSignal, onMount, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "../styles/global.css";
import Nav from "./nav";
import { DriverStanding } from "./types/interfaces";
import { cachedStandings, setCachedStandings } from "./types/store";

export default function Drivers() {
  const [Standings, setStandings] = createSignal<DriverStanding[]>([]);

  const fetchDriverStandings = async () => {
    try {
      const result = await invoke<DriverStanding[]>("get_drivers_standings");
      setStandings(result);
      setCachedStandings(result);
    } catch (error) {
      console.error("Error fetching driver standings:", error);
    }
  };

  onMount(async () => {
    if (cachedStandings()) {
      setStandings(cachedStandings()!);
      return;
    }
    await fetchDriverStandings();
  });

  return (
    <>
      <Nav />
      <h1 class="title">Driver Standings</h1>
      <div class="drivers-page">
        <table class="drivers-table">
          <thead>
            <tr>
              <th>Position</th>
              <th>Driver</th>
              <th>Team</th>
              <th>Points</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <Show
              when={Standings() && Standings().length > 0}
              fallback={
                <tr>
                  <td colspan={5} style="padding: 8rem;">
                    <div style="display: flex; justify-content: center; align-items: center; height: 100%;">
                      <div class="spinner"></div>
                    </div>
                  </td>
                </tr>
              }
            >
              <For each={Standings()}>
                {(d) => (
                  <tr>
                    <td>{d.position}</td>
                    <td>{d.driverName}</td>
                    <td>{d.teamName}</td>
                    <td>{d.points}</td>
                    <td>
                      {d.differenceToLeader ? `+${d.differenceToLeader}` : ""}
                    </td>
                  </tr>
                )}
              </For>
            </Show>
          </tbody>
        </table>
      </div>
    </>
  );
}
