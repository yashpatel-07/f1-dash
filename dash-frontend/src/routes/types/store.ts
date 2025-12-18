import { createSignal } from "solid-js";
import { DriverStanding, RaceStatus, ConstructorStatus } from "./interfaces";

const [cachedStandings, setCachedStandings] = createSignal<
  DriverStanding[] | null
>(null);
const [cachedRaceStatuses, setCachedRaceStatuses] = createSignal<
  RaceStatus[] | null
>(null);
const [cachedConstructorStandings, setCachedConstructorStandings] =
  createSignal<ConstructorStatus[] | null>(null);

export {
  cachedStandings,
  setCachedStandings,
  cachedRaceStatuses,
  setCachedRaceStatuses,
  cachedConstructorStandings,
  setCachedConstructorStandings,
};
