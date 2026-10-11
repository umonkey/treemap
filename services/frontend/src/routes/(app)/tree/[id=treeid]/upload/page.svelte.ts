import { getTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import type { ITree } from '$lib/types';
import { error } from '@sveltejs/kit';

class PageState {
	tree = $state<ITree>(DEFAULT_TREE);
	treeId = $state<string>('');
	busy = $state<boolean>(false);
	uploads = $state<string[]>([]);
	canSubmit = $state<boolean>(false);
	hasFiles = $state<boolean>(false);
	loading = $state<boolean>(false);
	error = $state<string | null>(null);

	async reload(id: string) {
		if (!id) {
			error(404);
		}

		this.treeId = id;
		this.loading = true;
		const { status, data, error: apiError } = await getTree(id, true);

		if (status === 200 && data) {
			this.tree = data;
			this.error = null;
		} else if (apiError) {
			this.error = apiError.description;
		} else {
			error(status);
		}
		this.loading = false;
	}

	handleBusy = (value: boolean) => {
		this.busy = value;
		this.canSubmit = !value && this.uploads.length > 0;
	};

	handleChange = (filesCount: number) => {
		this.hasFiles = filesCount > 0;
		this.busy = false;
		this.canSubmit = true;
	};
}

export const pageState = new PageState();
