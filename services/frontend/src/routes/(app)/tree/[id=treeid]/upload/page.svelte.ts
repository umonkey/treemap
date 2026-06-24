import { changeTreeThumbnail, getTree } from '$lib/api/trees';
import { deleteFile } from '$lib/api/uploads';
import { DEFAULT_TREE } from '$lib/constants';
import { showError } from '$lib/errors';
import type { ITree, ITreeFile } from '$lib/types';
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

	handleMakeThumbnail = async (file: ITreeFile) => {
		const res = await changeTreeThumbnail(this.tree.id, file.id);

		if (res.status >= 400) {
			showError(res.error?.description || `Error ${res.status} changing thumbnail.`);
		}
	};

	handleDelete = async (id: string) => {
		const res = await deleteFile(id);

		if (res.status >= 400) {
			showError(res.error?.description || `Error ${res.status} deleting file.`);
		}
	};
}

export const pageState = new PageState();
