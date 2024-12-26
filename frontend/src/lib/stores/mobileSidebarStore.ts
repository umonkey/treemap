import { writable, derived } from 'svelte/store';

export const mobileSidebarStore = writable<boolean>(false);
export const isSidebarVisible = derived(mobileSidebarStore, ($state) => $state);
