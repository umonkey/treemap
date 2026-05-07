import { getRecentComments } from '$lib/api/comments';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { IComment } from '$lib/types';

class PageState {
	comments = $state<IComment[]>([]);
	loading = $state<boolean>(true);
	error = $state<boolean>(false);

	load = async () => {
		this.loading = true;
		try {
			const { status, data } = await getRecentComments();

			if (status === 200 && data) {
				addUsers(data.users);
				addTrees(data.trees);
				this.comments = data.comments;
				this.error = false;
			} else {
				this.error = true;
			}
		} catch (err) {
			console.error('Failed to load comments:', err);
			this.error = true;
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
