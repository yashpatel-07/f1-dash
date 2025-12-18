import { For, createSignal, onMount, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { ConstructorStatus } from "./types/interfaces";
import {
  cachedConstructorStandings,
  setCachedConstructorStandings,
} from "./types/store";
import "../styles/global.css";
import Nav from "./nav";

export default function teams() {
  const [constructorStandings, setConstructorStandings] = createSignal<
    ConstructorStatus[]
  >([]);

  const fetchConstructorStandings = async () => {
    try {
      const standings: ConstructorStatus[] = await invoke(
        "get_constructors_standings"
      );
      setConstructorStandings(standings);
      setCachedConstructorStandings(standings);
    } catch (error) {
      console.error("Error fetching constructor standings:", error);
    }
  };

  onMount(() => {
    if (cachedConstructorStandings()) {
      setConstructorStandings(cachedConstructorStandings()!);
      return;
    }
    fetchConstructorStandings();
  });

  return (
    <>
      <Nav />
      <h1 class="title">Constructor Standings</h1>
      <div class="teams-page">
        <table class="teams-table">
          <thead>
            <tr>
              <th>Position</th>
              <th>Team</th>
              <th>Points</th>
              <th>Wins</th>
            </tr>
          </thead>
          <tbody>
            <Show
              when={constructorStandings() && constructorStandings().length > 0}
              fallback={
                <tr>
                  <td colspan={4} style="padding: 8rem;">
                    <div
                      style="
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100%;
                      "
                    >
                      <div class="spinner"></div>
                    </div>
                  </td>
                </tr>
              }
            >
              <For each={constructorStandings()}>
                {(team) => (
                  <tr>
                    <td>{team.position}</td>
                    <td>{team.teamName}</td>
                    <td>{team.points}</td>
                    <td>{team.wins}</td>
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
