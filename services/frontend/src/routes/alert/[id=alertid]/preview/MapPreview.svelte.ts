import { getAlert, getAlertPhotos, type IAlert } from '$lib/api/alerts';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

class PreviewState {
	alert = $state<IAlert>();
	photos = $state<string[]>([]);
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
		this.alert = undefined;
		this.photos = [];
		this.expand = false;

		mapBus.emit('pin', undefined);
	};

	public reload = (id: string) => {
		console.debug(`Reloading preview for alert ${id}`);
		this.loading = true;

		getAlert(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.alert = res.data;

				if (this.alert.lat !== null && this.alert.lon !== null) {
					mapBus.emit('pin', {
						lat: this.alert.lat,
						lng: this.alert.lon
					});
				}
			} else if (res.error) {
				showError(res.error.description);
			}
			this.loading = false;
		});

		getAlertPhotos(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.photos = res.data;
			} else {
				this.photos = [];
			}
		});
	};

	private handlePreviewSignal = (id: string | undefined) => {
		if (!id) {
			this.handleClose();
		}
	};

	public onMount = () => {
		mapBus.on('preview', this.handlePreviewSignal);

		return () => {
			mapBus.off('preview', this.handlePreviewSignal);
			this.clear();
		};
	};
}

export const previewState = new PreviewState();
