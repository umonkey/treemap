import { getComments } from '$lib/api/comments';
import { addUsers } from '$lib/stores/userStore';
import type { IComment } from '$lib/types';

export class PageState {
	id = $state<string>();
	comments = $state<IComment[]>([]);

	public reload = (id: string) => {
		this.id = id;
		this.comments = [];

		this.loadComments();
	};

	public loadComments = () => {
		if (!this.id) {
			return;
		}

		getComments('tree', this.id).then((res) => {
			if (res.status === 200 && res.data) {
				addUsers(res.data.users);
				this.comments = res.data.comments.sort((a, b) => b.added_at - a.added_at);
			}
		});
	};
}
