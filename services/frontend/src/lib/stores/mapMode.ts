import { writable } from 'svelte/store';

export type MapMode = 'move' | undefined;

export const mapMode = writable<MapMode>(undefined);
