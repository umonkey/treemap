import { getActiveAlertsGeoJSON, type IAlertCollection } from '$lib/api/alerts';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { Debouncer } from '$lib/utils/debounce';
import { getMapContext } from 'svelte-maplibre';

class AlertLayerState {
	markers = $state.raw<IAlertCollection | undefined>(undefined);
	fetchDebouncer = new Debouncer(100);

	private reload = () => {
		this.fetchDebouncer.run(() => {
			getActiveAlertsGeoJSON()
				.then(({ status, data }) => {
					if (status === 200 && data) {
						console.debug(`[AlertLayer] Received ${data.features.length} alerts.`);
						this.markers = data;
					}
				})
				.catch((e) => {
					console.error('Error loading alerts.', e);
					showError('Error loading alerts, please try again.');
				});
		});
	};

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public handleClick = async (e: any) => {
		if (!e.features || e.features.length === 0) {
			return;
		}

		const feature = e.features[0];
		const alertId = feature.properties.id;

		const [lng, lat] = feature.geometry.coordinates;
		mapBus.emit('move', { lat, lng });

		console.debug(`[AlertLayer] Alert ${alertId} clicked.`);

		await goto(routes.alertPreview(alertId));

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}
	};

	public onMount = () => {
		const map = getMapContext()?.map;

		if (!map) {
			console.warn('Map not available, cannot display alerts.');
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

export const alertLayerState = new AlertLayerState();
