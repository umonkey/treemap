import { get, writable } from 'svelte/store';

export const hooks = () => {
	const element = writable<HTMLElement | null>(null);

	const handleExpand = (url: string) => {
		window.open(url, '_blank');
	};

	const scroll = (offset: number) => {
		const em = get(element);

		if (em === null) {
			console.debug('[gallery] Element not set, cannot scroll.');
			return;
		}

		em.scrollBy({
			left: offset,
			behavior: 'smooth'
		});
	};

	const handleLeft = () => {
		scroll(-40);
	};

	const handleRight = () => {
		scroll(40);
	};

	return { handleExpand, handleLeft, handleRight, element };
};
