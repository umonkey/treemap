import type { IStats } from '$lib/types';
import { writable } from 'svelte/store';

export const statsStore = writable<IStats | null>(null);
