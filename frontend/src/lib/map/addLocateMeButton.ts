/**
 * Adds a "Locate Me" button.
 *
 * Reads current position from the locationStore.
 * Upon button click, pans the map to the last received position.
 */

import L from 'leaflet';
import { Map } from 'leaflet';
import { locationStore } from '$lib/stores/locationStore';
import type { IMyPosition } from '$lib/types';

export const addLocateMeButton = (map: Map) => {
	console.debug('[map] Adding the Locate Me button.');

	// Last received position, to pan to on button click.
	let lastPosition: IMyPosition | null = null;
	let visibility = 'none';

	const unsubscribe = locationStore.subscribe((pos) => {
		console.debug('[map] LocateMe position:', pos);
		lastPosition = pos;
		visibility = pos !== null ? 'block' : 'none';

		const element = document.getElementsByClassName('leaflet-locate-me-container')[0] ?? undefined;

		if (element) {
			element.style.display = visibility;
		}
	});

	L.Control.LocateMeButton = L.Control.extend({
		options: {
			position: 'topleft'
		},

		onAdd: function (map: Map) {
			const container = L.DomUtil.create(
				'div',
				'leaflet-bar leaflet-control leaflet-locate-me-container'
			);
			container.style.display = visibility;

			const button = L.DomUtil.create('a', 'leaflet-locate-button', container);

			button.href = '#';
			button.type = 'button';

			const image = L.DomUtil.create('img', 'leaflet-control-button-icon', button);
			image.src = '/icons/marker.svg';
			image.width = 20;
			image.height = 30;

			L.DomEvent.disableClickPropagation(button);

			L.DomEvent.on(button, 'click', (e) => {
				e.preventDefault();

				if (lastPosition) {
					map.panTo([lastPosition.lat, lastPosition.lng]);
				}
			});

			return container;
		}
	});

	const control = new L.Control.LocateMeButton();
	const button = control.addTo(map);

	console.debug('[map] LocateMe button added.', button);

	map.on('unload', () => {
		console.debug('[map] Removing the Locate Me button.');
		unsubscribe();
	});
};