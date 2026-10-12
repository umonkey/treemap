import { getTreeComments } from '$lib/api/comments';
import { getObservations } from '$lib/api/observations';
import { mapBus } from '$lib/buses/mapBus';
import { goto, routes } from '$lib/routes';
import { mapMode } from '$lib/stores/mapMode';
import type { IComment, IObservation } from '$lib/types';

class PreviewContentState {
	observations = $state<IObservation | null>(null);
	comments = $state<IComment[]>([]);
	expand = $state<boolean>(false);
	loading = $state<boolean>(false);

	public toggleExpand = () => {
		this.expand = !this.expand;
	};

	public handleClose = async () => {
		this.clear();
		await goto(routes.map());
	};

	public clear = () => {
		this.observations = null;
		this.comments = [];
		this.expand = false;
	};

	public reload = (id: string) => {
		console.debug(`Reloading preview for tree ${id}`);

		getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observations = res.data;
			} else {
				this.observations = null;
			}
		});

		getTreeComments(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.comments = res.data.comments.sort((a, b) => b.added_at - a.added_at);
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

	public init = () => {
		mapBus.on('preview', this.handlePreviewSignal);
		mapMode.set('preview');

		return () => {
			mapBus.off('preview', this.handlePreviewSignal);
			mapMode.set(undefined);
			this.clear();
		};
	};
}

export const previewContentState = new PreviewContentState();
