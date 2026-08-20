import { writable } from 'svelte/store';

export type MapMode = 'move' | 'add' | 'add-row' | 'preview' | 'search' | undefined;

export const mapMode = writable<MapMode>(undefined);
