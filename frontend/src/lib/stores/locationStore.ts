import type { IMyPosition } from '$lib/types';
import { writable } from 'svelte/store';

/**
 * The user's current location, updated by LocationControl.
 * This is transient and not persisted to localStorage.
 */
export const locationStore = writable<IMyPosition | null>(null);
