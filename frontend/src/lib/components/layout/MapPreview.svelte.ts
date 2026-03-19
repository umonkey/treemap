import type { ITree, IObservation, IComment } from '$lib/types';
import { mapBus } from '$lib/buses';
import { apiClient } from '$lib/api';
import { menuState } from '$lib/stores/treeMenu';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

class PreviewState {
	tree = $state<ITree>();
	observations = $state<IObservation | null>(null);
	comments = $state<IComment[]>([]);
	expand = $state<boolean>(false);

	public toggleExpand = () => {
		this.expand = !this.expand;
	};

	public handleClose = async () => {
		this.tree = undefined;
		this.observations = null;
		this.comments = [];
		this.expand = false;

		mapBus.emit('pin', undefined);

		await goto(routes.map());
	};

	public handleContextMenu = () => {
		menuState.set(true);
	};

	private handleTreeSelect = (id: string | undefined) => {
		if (!id) {
			this.tree = undefined;
			return;
		}

		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.tree = res.data;

				mapBus.emit('pin', {
					lat: this.tree.lat,
					lng: this.tree.lon
				});
			} else if (res.error) {
				showError(res.error.description);
			}
		});

		apiClient.getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observations = res.data;
			} else {
				this.observations = null;
			}
		});

		apiClient.getTreeComments(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.comments = res.data.comments;
			} else {
				this.comments = [];
			}
		});
	};

	public onMount = () => {
		mapBus.on('preview', this.handleTreeSelect);

		return () => {
			mapBus.off('preview', this.handleTreeSelect);
		};
	};
}

export const previewState = new PreviewState();
