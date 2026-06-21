import { addComment, getTreeComments } from '$lib/api/comments';
import { getObservations } from '$lib/api/observations';
import { getTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError, showInfo } from '$lib/errors';
import { locale } from '$lib/locale';
import { addUsers } from '$lib/stores/userStore';
import type { IComment, ILatLng, IObservation, ITree } from '$lib/types';

class PageState {
	id = $state<string>();
	tree = $state<ITree>();
	comments = $state<IComment[]>([]);
	observation = $state<IObservation>();

	public reload = (id: string) => {
		this.id = id;
		this.tree = undefined;
		this.comments = [];
		this.observation = undefined;

		getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.tree = res.data;

				this.moveMap({
					lat: res.data.lat,
					lng: res.data.lon
				});
			} else if (res.error) {
				showError(res.error.description);
			}
		});

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
		const treeId = this.tree?.id;

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

	private moveMap = (ll: ILatLng) => {
		console.debug(`Moving map center once to ${ll.lat},${ll.lng}`);

		mapBus.emit('map-once', ll);
	};
}

export const pageState = new PageState();
