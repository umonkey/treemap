import { addComment, getTreeComments } from '$lib/api/comments';
import { getObservations } from '$lib/api/observations';
import { showError, showInfo } from '$lib/errors';
import { locale } from '$lib/locale';
import { addUsers } from '$lib/stores/userStore';
import type { IComment, IObservation } from '$lib/types';

class PageState {
	id = $state<string>();
	comments = $state<IComment[]>([]);
	observation = $state<IObservation>();

	public reload = (id: string) => {
		this.id = id;
		this.comments = [];
		this.observation = undefined;

		this.loadComments();

		getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observation = res.data;
			}
		});
	};

	public loadComments = () => {
		if (!this.id) {
			return;
		}

		getTreeComments(this.id).then((res) => {
			if (res.status === 200 && res.data) {
				addUsers(res.data.users);
				this.comments = res.data.comments.sort((a, b) => b.added_at - a.added_at);
			}
		});
	};

	public handleSubmitComment = async (message: string): Promise<boolean> => {
		const treeId = this.id;

		if (!treeId) {
			return false;
		}

		try {
			const res = await addComment(treeId, message);
			if (res.status >= 200 && res.status < 300) {
				showInfo(locale.toastCommentAdded());
				this.loadComments();
				return true;
			} else {
				console.error(`Error ${res.status} adding a comment.`, res);
				showError(res.error?.description || `Error ${res.status} adding a comment.`);
				return false;
			}
		} catch (e) {
			console.error('Exception while adding a comment.', e);
			showError('Exception adding a comment.');
			return false;
		}
	};
}

export const pageState = new PageState();
