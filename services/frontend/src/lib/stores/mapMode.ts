import { writable } from 'svelte/store';

export type MapMode = 'move' | 'add' | undefined;

export const mapMode = writable<MapMode>(undefined);
