import type { MountFn } from '$lib/types';
import { db, type IUpload } from '$lib/db';
import { writable } from 'svelte/store';
import { liveQuery } from 'dexie';

/**
 * Loads the list of uploads from the IndexedDB.
 *
 * Uses Dexie's liveQuery to keep the list updated.
 * Returns a function to unsubscribe from the query.
 */
export const hooks = (mount: MountFn) => {
	const uploads = writable<IUpload[]>([]);

	mount(() => {
		const query = liveQuery(() => db.uploads.orderBy('created_at').reverse().toArray());

		const subscription = query.subscribe({
			next: (data) => {
				uploads.set(data);
			},
			error: (err) => console.error('Failed to load uploads:', err)
		});

		return () => subscription.unsubscribe();
	});

	return { uploads };
};
