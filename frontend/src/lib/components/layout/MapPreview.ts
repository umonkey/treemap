import type { ITree } from '$lib/types';
import { apiClient } from '$lib/api';
import { goto } from '$lib/routes';
import { mapHome } from '$lib/map';
import { menuState } from '$lib/stores/treeMenu';
import { writable } from 'svelte/store';
import { mapBus } from '$lib/buses';
import type { MountFn } from '$lib/types';

export const hook = ({ onMount }: { onMount: MountFn }) => {
	const visible = writable<boolean>(false);
	const tree = writable<ITree | null>(null);
	const error = writable<string | null>(null);

	const handleClose = () => {
		goto(mapHome());
	};

	const reload = (id: string | null) => {
		console.debug(`[MapPreview] Selected tree: ${id}`);

		if (id === null) {
			visible.set(false);
			tree.set(null);
			return;
		}

		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				tree.set(res.data);
				visible.set(true);
			} else if (res.error) {
				error.set(res.error.description);
			}
		});
	};

	const handleContextMenu = () => {
		menuState.set(true);
	};

	// A third party asks to close the preview.
	const handleClosePreview = () => {
		visible.set(false);
	};

	onMount(() => {
		mapBus.on('closePreview', handleClosePreview);

		return () => {
			mapBus.off('closePreview', handleClosePreview);
		};
	});

	return { visible, error, tree, reload, handleClose, handleContextMenu };
};
