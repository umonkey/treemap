import { getReport, getReportPhotos, type IReport } from '$lib/api/reports';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

class PreviewState {
	report = $state<IReport>();
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
		this.report = undefined;
		this.photos = [];
		this.expand = false;

		mapBus.emit('pin', undefined);
	};

	public reload = (id: string) => {
		console.debug(`Reloading preview for report ${id}`);
		this.loading = true;

		getReport(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.report = res.data;

				if (this.report.lat !== null && this.report.lon !== null) {
					mapBus.emit('pin', {
						lat: this.report.lat,
						lng: this.report.lon
					});
				}
			} else if (res.error) {
				showError(res.error.description);
			}
			this.loading = false;
		});

		getReportPhotos(id).then((res) => {
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
