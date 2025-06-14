import { routes, goto } from '$lib/routes';
import { writable } from 'svelte/store';
import { searchStore } from '$lib/stores';

export const hooks = () => {
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

	return { focused, handleReset, handleFocus, handleBlur, handleChange, value: searchStore };
};
