import { deleteComment } from '$lib/api/comments';
import { showError, showInfo } from '$lib/errors';
import { locale } from '$lib/locale';
import { authStore } from '$lib/stores/authStore';
import { formatDate } from '$lib/utils/strings';
import { get } from 'svelte/store';

class CommentLogic {
	isDeleting = $state<boolean>(false);

	constructor() {
		// Pure constructor
	}

	getFormattedDate = (addedAt: number) => {
		return formatDate(addedAt);
	};

	canDelete = (addedBy: string) => {
		const currentUser = get(authStore);
		return currentUser?.id === addedBy;
	};

	handleDelete = async (
		treeId: number,
		commentId: string,
		onDeleteCallback?: (commentId: string) => void
	) => {
		if (this.isDeleting) {
			return;
		}

		const confirmed = confirm(locale.commentDeleteConfirm());
		if (!confirmed) {
			return;
		}

		this.isDeleting = true;

		try {
			const res = await deleteComment(treeId, commentId);
			if (res.status >= 200 && res.status < 300) {
				showInfo(locale.toastCommentDeleted());
				if (onDeleteCallback) {
					onDeleteCallback(commentId);
				}
			} else {
				showError(res.error?.description || `Error ${res.status} deleting comment`);
			}
		} catch (err) {
			console.error('Exception deleting comment:', err);
			showError('Exception deleting comment');
		} finally {
			this.isDeleting = false;
		}
	};
}

export const componentState = new CommentLogic();
