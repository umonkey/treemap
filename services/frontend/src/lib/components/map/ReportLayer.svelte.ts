import { getActiveReportsGeoJSON, type IReportCollection } from '$lib/api/reports';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { Debouncer } from '$lib/utils/debounce';
import { getMapContext } from 'svelte-maplibre';

class ReportLayerState {
	markers = $state.raw<IReportCollection | undefined>(undefined);
	fetchDebouncer = new Debouncer(100);

	private reload = () => {
		this.fetchDebouncer.run(() => {
			getActiveReportsGeoJSON()
				.then(({ status, data }) => {
					if (status === 200 && data) {
						console.debug(`[ReportLayer] Received ${data.features.length} reports.`);
						this.markers = data;
					}
				})
				.catch((e) => {
					console.error('Error loading reports.', e);
					showError('Error loading reports, please try again.');
				});
		});
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = async (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const reportId = feature.properties.id;

		const [lng, lat] = feature.geometry.coordinates;
		mapBus.emit('move', { lat, lng });

		console.debug(`[ReportLayer] Report ${reportId} clicked.`);

		await goto(routes.reportPreview(reportId));

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}
	};

	public onMount = () => {
		const map = getMapContext()?.map;

		if (!map) {
			console.warn('Map not available, cannot display reports.');
			return;
		}

		const reload = () => this.reload();

		mapBus.on('reload', reload);

		reload();

		return () => {
			mapBus.off('reload', reload);
		};
	};
}

export const reportLayerState = new ReportLayerState();
