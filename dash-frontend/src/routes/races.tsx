import { For, Show, createMemo, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "../styles/global.css";
import Nav from "./nav";
import { RaceStatus } from "./types/interfaces";
import { cachedRaceStatuses, setCachedRaceStatuses } from "./types/store";

const [isShowAllRaces, setIsShowAllRaces] = createSignal(false);

export default function Races() {
  const [raceStatuses, setRaceStatuses] = createSignal<RaceStatus[]>([]);

  const fetchRaceStatuses = async () => {
    try {
      const result = await invoke<RaceStatus[]>("get_race_schedule");
      setRaceStatuses(result);
      setCachedRaceStatuses(result);
    } catch (error) {
      console.error("Error fetching race schedule:", error);
    }
  };

  onMount(async () => {
    if (cachedRaceStatuses()) {
      setRaceStatuses(cachedRaceStatuses()!);
      return;
    }
    await fetchRaceStatuses();
  });

  const nextRace = () => {
    const today = new Date();
    return raceStatuses().find((r) => new Date(r.date) >= today);
  };

  const toAmPm = (utc: string) => {
    return new Date(`1970-01-01T${utc}`).toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    const day = date.getDate();
    const month = date.toLocaleString("default", { month: "long" });

    const getOrdinal = (n: number) => {
      if (n > 3 && n < 21) return "th";
      switch (n % 10) {
        case 1:
          return "st";
        case 2:
          return "nd";
        case 3:
          return "rd";
        default:
          return "th";
      }
    };
    return `${day}${getOrdinal(day)} ${month.slice(0, 3)}`;
  };

  return (
    <>
      <Nav />
      <h1 class="title">Race Schedule</h1>
      <Show
        when={raceStatuses() && raceStatuses().length > 0}
        fallback={
          <div style="padding: 8rem; display: flex; justify-content: center; align-items: center; height: 100%;">
            <div class="spinner"></div>
          </div>
        }
      >
        <Show
          when={nextRace()}
          fallback={
            <div>
              <p style="text-align: center; padding: 2rem; font-size: 1.2rem;">
                No upcoming races.
              </p>
              <div class="race-toggle-container">
                <button
                  class="btn-f1"
                  onClick={() => setIsShowAllRaces(!isShowAllRaces())}
                >
                  {isShowAllRaces() === false ? "Show All Races" : "Show Less"}
                </button>
              </div>
            </div>
          }
        >
          {(race) => {
            const r = race(); // call the accessor to get RaceStatus

            const sessions = [
              r.firstPractice && { name: "FP1", session: r.firstPractice },
              r.secondPractice && { name: "FP2", session: r.secondPractice },
              r.thirdPractice && { name: "FP3", session: r.thirdPractice },
              r.sprintQualifying && {
                name: "Sprint Qualifying",
                session: r.sprintQualifying,
              },
              r.sprint && { name: "Sprint", session: r.sprint },
              r.qualifying && { name: "Qualifying", session: r.qualifying },
            ].filter(Boolean) as {
              name: string;
              session: { date?: string; time?: string };
            }[];

            return (
              <div>
                <div class="race-card upcoming">
                  <h2>Round {r.round} — Upcoming!</h2>
                  <div class="race-info-grid">
                    <div>
                      <p>
                        <strong>Race:</strong> {r.raceName}
                      </p>
                      <p>
                        <strong>Date:</strong> {formatDate(r.date)}
                      </p>
                      <p>
                        <strong>Time:</strong> {toAmPm(r.time)}
                      </p>
                    </div>
                    <div>
                      <ul class="session-list">
                        <For each={sessions}>
                          {(s) => (
                            <li class="session-list-inner">
                              <strong>{s.name}:</strong>{" "}
                              {formatDate(s.session.date!)}{" "}
                              {toAmPm(s.session.time!)}
                            </li>
                          )}
                        </For>
                      </ul>
                    </div>
                  </div>
                  <p class="note">* All times shown are track times.</p>
                </div>
                <div class="race-toggle-container">
                  <button
                    class="btn-f1"
                    onClick={() => setIsShowAllRaces(!isShowAllRaces())}
                  >
                    {isShowAllRaces() === false ? "Show All" : "Show Less"}
                  </button>
                </div>
              </div>
            );
          }}
        </Show>
      </Show>

      <Show when={isShowAllRaces()}>
        <div class="races-page">
          <div class="race-grid">
            <For each={raceStatuses()}>
              {(race) => (
                <div class="race-card">
                  <h2>Round {race.round}</h2>
                  <p>
                    <strong>Race:</strong> {race.raceName}
                  </p>
                  <p>
                    <strong>Date:</strong> {race.date}
                  </p>
                  <p>
                    <strong>Time:</strong> {toAmPm(race.time)}
                  </p>
                </div>
              )}
            </For>
          </div>
        </div>
      </Show>
    </>
  );
}
