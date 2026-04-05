import { getTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import type { ITree, ITreeFile } from '$lib/types';

class ComponentState {
	loading = $state<boolean>(true);
	error = $state<string>('');
	tree = $state<ITree>(DEFAULT_TREE);
	files = $state<ITreeFile[]>([]);

	async reload(id: string) {
		this.loading = true;

		try {
			const res = await getTree(id, true);

			if (res.status === 200 && res.data) {
				this.tree = res.data;
				this.files = res.data.files;
				this.error = '';
			} else if (res.error) {
				this.error = res.error.description;
			}
		} catch (err) {
			this.error = String(err);
		} finally {
			this.loading = false;
		}
	}
}

export const galleryPreviewState = new ComponentState();
