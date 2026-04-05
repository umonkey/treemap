import { searchStore } from '$lib/stores/searchStore';
import { previewState } from '$lib/components/layout/MapPreview.svelte.ts';

class PageState {
	search = $state<string | undefined>(undefined);
	preview = $state<string | undefined>(undefined);

	public updateSearch = (value: string | undefined) => {
		console.debug(`Updating search query to: ${value}`);
		searchStore.set(value);
	};

	public updatePreview = (value: string | undefined) => {
		if (value) {
			console.debug(`Updating preview to: ${value}`);
			previewState.reload(value);
		}
	};
}

export const pageState = new PageState();
