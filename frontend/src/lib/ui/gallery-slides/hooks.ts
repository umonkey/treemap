import type { IGalleryItem } from '$lib/types';
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

	const scrollToId = (id: string, items: IGalleryItem[]) => {
		const em = get(element);

		if (em === null) {
			return;
		}

		const index = items.findIndex((s) => s.id === id);
		if (index === -1) {
			return;
		}

		const slideElement = em.children[index] as HTMLElement;
		if (slideElement) {
			slideElement.scrollIntoView({
				behavior: 'instant',
				block: 'nearest',
				inline: 'start'
			});
		}
	};

	return { handleExpand, handleLeft, handleRight, element, scrollToId };
};
