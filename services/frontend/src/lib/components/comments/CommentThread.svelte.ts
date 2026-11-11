import { getComments } from '$lib/api/comments';
import { addUsers } from '$lib/stores/userStore';
import type { CommentEntityType, IComment } from '$lib/types';

export class CommentThreadState {
	comments = $state<IComment[]>([]);
	entityType = $state<CommentEntityType>();
	entityId = $state<string>();

	public reload = async (entityType: CommentEntityType, entityId: string) => {
		this.entityType = entityType;
		this.entityId = entityId;
		this.comments = [];

		await this.load();
	};

	public load = async () => {
		if (!this.entityType || !this.entityId) {
			return;
		}

		const res = await getComments(this.entityType, this.entityId);

		if (res.status === 200 && res.data) {
			addUsers(res.data.users);
			this.comments = res.data.comments.sort((a, b) => b.added_at - a.added_at);
		}
	};
}
