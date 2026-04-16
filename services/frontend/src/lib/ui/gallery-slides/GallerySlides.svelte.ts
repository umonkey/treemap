import type { IGalleryItem } from '$lib/types';

class GalleryState {
	element = $state<HTMLElement | null>(null);

	handleExpand = (url: string) => {
		window.open(url, '_blank');
	};

	private scroll = (offset: number) => {
		if (this.element === null) {
			console.debug('[gallery] Element not set, cannot scroll.');
			return;
		}

		this.element.scrollBy({
			left: offset,
			behavior: 'smooth'
		});
	};

	handleLeft = () => {
		this.scroll(-40);
	};

	handleRight = () => {
		this.scroll(40);
	};

	scrollToId = (id: string, items: IGalleryItem[]) => {
		if (this.element === null) {
			return;
		}

		const index = items.findIndex((s) => s.id === id);
		if (index === -1) {
			return;
		}

		const slideElement = this.element.children[index] as HTMLElement;
		if (slideElement) {
			slideElement.scrollIntoView({
				behavior: 'instant',
				block: 'nearest',
				inline: 'start'
			});
		}
	};
}

export const galleryState = new GalleryState();
