import { createSignal } from "solid-js";
import { DriverStanding, RaceStatus } from "./interfaces";

const [cachedStandings, setCachedStandings] = createSignal<
  DriverStanding[] | null
>(null);
const [cachedRaceStatuses, setCachedRaceStatuses] = createSignal<
  RaceStatus[] | null
>(null);

export {
  cachedStandings,
  setCachedStandings,
  cachedRaceStatuses,
  setCachedRaceStatuses,
};
