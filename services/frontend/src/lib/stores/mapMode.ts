import { writable } from 'svelte/store';

export type MapMode = 'move' | 'add' | 'add-row' | undefined;

export const mapMode = writable<MapMode>(undefined);
