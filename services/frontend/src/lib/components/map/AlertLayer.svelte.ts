import { getActiveAlertsGeoJSON, type IAlertCollection } from '$lib/api/alerts';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { mapPoiStore } from '$lib/stores/mapPoi.svelte';
import { Debouncer } from '$lib/utils/debounce';
import { getMapContext } from 'svelte-maplibre';
import { get } from 'svelte/store';

export class AlertLayerLogic {
	markers = $state.raw<IAlertCollection | undefined>(undefined);
	enabled = $state<boolean>(true);
	fetchDebouncer = new Debouncer(100);

	get alerts(): boolean {
		return this.enabled;
	}

	constructor() {
		this.enabled = get(mapLayerStore).alerts !== false;
	}

	private reload = () => {
		if (!this.enabled) {
			return;
		}

		this.fetchDebouncer.run(() => {
			getActiveAlertsGeoJSON()
				.then(({ status, data }) => {
					if (status === 200 && data) {
						console.debug(`[AlertLayer] Received ${data.features.length} alerts.`);
						this.markers = data;
						mapPoiStore.alerts = data.features.map((f) => ({
							lat: f.geometry.coordinates[1],
							lon: f.geometry.coordinates[0],
							url: routes.alertPreview(f.properties.id)
						}));
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

		const unsub = mapLayerStore.subscribe((layers) => {
			const wasEnabled = this.enabled;
			this.enabled = layers.alerts !== false;
			if (!wasEnabled && this.enabled && !this.markers) {
				this.reload();
			}
		});

		mapBus.on('reload', reload);

		if (this.enabled) {
			this.reload();
		}

		return () => {
			unsub();
			mapBus.off('reload', reload);
			mapPoiStore.alerts = [];
		};
	};
}

export type AlertLayerState = AlertLayerLogic;
