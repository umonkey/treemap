import { getTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import { routes } from '$lib/routes';
import type { ITree, ITreeFile } from '$lib/types';

export class ComponentState {
	loading = $state<boolean>(true);
	error = $state<string>('');
	tree = $state<ITree>(DEFAULT_TREE);
	images = $state<string[]>([]);

	async reload(id: string) {
		this.loading = true;

		try {
			const res = await getTree(id, true);

			if (res.status === 200 && res.data) {
				this.tree = res.data;
				this.images = res.data.files.map((file: ITreeFile): string => routes.file(file.small_id));
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

export const galleryState = new ComponentState();
