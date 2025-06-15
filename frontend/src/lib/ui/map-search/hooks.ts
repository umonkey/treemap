import { routes, goto } from '$lib/routes';
import { writable } from 'svelte/store';
import { searchStore } from '$lib/stores';
import { getMap } from '$lib/map';
import type { MountFn } from '$lib/types';

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	const focused = writable<boolean>(false);

	const handleReset = () => {
		goto(routes.map());
	};

	const handleFocus = () => {
		focused.set(true);
	};

	const handleBlur = () => {
		setTimeout(() => {
			focused.set(false);
		}, 100);
	};

	const handleChange = (e: Event) => {
		const query = (e.target as HTMLInputElement).value;
		goto(routes.searchQuery(query));
	};

	onMount(() => {
		const map = getMap();

		map?.getContainer().classList.add('has-search');

		return () => {
			map?.getContainer().classList.remove('has-search');
		};
	});

	return { focused, handleReset, handleFocus, handleBlur, handleChange, value: searchStore };
};
