import { apiClient } from '$lib/api';
import { addUsers } from '$lib/stores/userStore';
import type { IComment, IObservation, ITree, ILatLng } from '$lib/types';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';

class PageState {
	id = $state<string>();
	tree = $state<ITree>();
	comments = $state<IComment[]>([]);
	observation = $state<IObservation>();

	public reload = (id: string) => {
		this.tree = undefined;
		this.comments = [];
		this.observation = undefined;

		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.tree = res.data;

				this.moveMap({
					lat: res.data.lat,
					lng: res.data.lon
				});
			}
		});

		apiClient.getTreeComments(id).then((res) => {
			if (res.status === 200 && res.data) {
				addUsers(res.data.users);
				this.comments = res.data.comments;
			}
		});

		apiClient.getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observation = res.data;
			}
		});
	};

	public handleSubmitComment = (message: string) => {
		const treeId = this.tree?.id;

		if (!treeId) {
			return;
		}

		apiClient
			.addComment(treeId, message)
			.then((res) => {
				if (res.status >= 200 && res.status < 300) {
					// OK
				} else {
					console.error(`Error ${res.status} adding a comment.`, res);
					showError(`Error ${res.status} adding a comment.`);
				}
			})
			.catch((e) => {
				console.error('Exception while adding a comment.', e);
				showError('Exception adding a comment.');
			});
	};

	private moveMap = (ll: ILatLng) => {
		console.debug(`Moving map center to ${ll.lat},${ll.lng}`);

		mapBus.emit('move', ll);
	};
}

export const pageState = new PageState();
