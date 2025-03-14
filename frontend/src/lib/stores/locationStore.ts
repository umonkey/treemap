import type { IMyPosition } from "$lib/types";
import { derived, writable } from "svelte/store";

export const locationStore = writable<IMyPosition | null>(null);
export const currentLocation = derived(locationStore, ($location) => $location);
