import { addComment } from '$lib/api/comments';
import { showError, showInfo } from '$lib/errors';
import { locale } from '$lib/locale';
import type { CommentEntityType } from '$lib/types';

export class CommentFormLogic {
	message = $state('');
	submitting = $state(false);

	public handleChange = (value: string) => {
		this.message = value;
	};

	public submit = async (entityType: CommentEntityType, entityId: string): Promise<boolean> => {
		if (!this.message || this.submitting) {
			return false;
		}

		this.submitting = true;

		try {
			const res = await addComment(entityType, entityId, this.message);

			if (res.status >= 200 && res.status < 300) {
				showInfo(locale.toastCommentAdded());
				this.message = '';
				return true;
			}

			console.error(`Error ${res.status} adding a comment.`, res);
			showError(res.error?.description || locale.toastErrorAddingComment());
			return false;
		} catch (e) {
			console.error('Exception while adding a comment.', e);
			showError(locale.toastErrorAddingComment());
			return false;
		} finally {
			this.submitting = false;
		}
	};
}
