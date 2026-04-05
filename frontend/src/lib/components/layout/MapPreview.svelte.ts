import { getTreeComments } from '$lib/api/comments';
import { getObservations } from '$lib/api/observations';
import { getTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import type { IComment, IObservation, ITree } from '$lib/types';

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
		if (this.tree) {
			menuBus.emit('show', this.tree.id);
		}
	};

	// Handle tree selection.
	// This is triggered via tree click on the map.
	// We don't want to handle this directly, instead
	// we just update the url and the rest happens on reload.
	private handleTreeSelect = async (id: string) => {
		await goto(routes.mapPreview(id));
	};

	public reload = (id: string) => {
		console.debug(`Reloading preview for tree ${id}`);

		getTree(id).then((res) => {
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

		getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observations = res.data;
			} else {
				this.observations = null;
			}
		});

		getTreeComments(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.comments = res.data.comments;
			} else {
				this.comments = [];
			}
		});
	};

	private handlePreviewSignal = (id: string | undefined) => {
		if (!id) {
			this.handleClose();
		}
	};

	public onMount = () => {
		mapBus.on('select', this.handleTreeSelect);
		mapBus.on('preview', this.handlePreviewSignal);

		return () => {
			mapBus.off('select', this.handleTreeSelect);
			mapBus.off('preview', this.handlePreviewSignal);
		};
	};
}

export const previewState = new PreviewState();
