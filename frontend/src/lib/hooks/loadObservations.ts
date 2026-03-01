import { writable } from 'svelte/store';
import { apiClient } from '$lib/api';
import type { IObservation, IError } from '$lib/types';

export const loadObservations = () => {
	const loading = writable(false);
	const error = writable<IError | null>(null);
	const observations = writable<IObservation | null>(null);

	const reload = async (treeId: string) => {
		loading.set(true);
		error.set(null);

		const response = await apiClient.getObservations(treeId);

		if (response.error) {
			error.set(response.error);
		} else {
			observations.set(response.data ?? null);
		}

		loading.set(false);
	};

	return {
		loading,
		error,
		observations,
		reload
	};
};
