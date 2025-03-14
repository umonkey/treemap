import { derived, writable } from "svelte/store";

export const mobileSidebarStore = writable<boolean>(false);
export const isSidebarVisible = derived(mobileSidebarStore, ($state) => $state);
