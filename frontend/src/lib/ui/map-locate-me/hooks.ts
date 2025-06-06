import L from 'leaflet';
import type { Map } from 'leaflet';
import { get, writable } from 'svelte/store';
import type { ILatLng, MountFn } from '$lib/types';
import { getContext, mapKey } from '$lib/map';
import ICON from '$lib/assets/locate.svg';
import './styles.css';

export const hooks = (mount: MountFn) => {
	// The button control, remove on unmount.
	const button = writable<L.Control | null>(null);

	// Map instance, set during initialization.
	const map = writable<Map | null>(null);

	// Last received position, to pan to.
	const lastPosition = writable<ILatLng | null>(null);

	// Handle the button click.
	const handleClick = (e: Event) => {
		e.preventDefault();

		const m = get(map);

		if (m === null) {
			console.debug('[MapAddTree] Map is not initialized');
			return;
		}

		const center = get(lastPosition);

		if (center !== null) {
			m.panTo([center.lat, center.lng]);
		}
	};

	// Adds the button to the map.
	const initialize = () => {
		const container = L.DomUtil.create('div', 'leaflet-bar leaflet-control');

		const button = L.DomUtil.create('a', 'locateMeButton', container);

		button.href = '#';
		button.type = 'button';

		const image = L.DomUtil.create('img', 'leaflet-control-button-icon', button);
		image.src = ICON;
		image.width = 30;
		image.height = 30;

		L.DomEvent.disableClickPropagation(button);

		// Clicking the button starts the process of adding a row of trees.
		L.DomEvent.on(button, 'click', handleClick);

		return container;
	};

	mount(() => {
		const m = getContext<Map>(mapKey) ?? null;

		if (m) {
			console.debug('[MapAddTree] Mounting.');

			map.set(m);

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			(L.Control as any).TreeButton = L.Control.extend({
				options: {
					position: 'topleft'
				},

				onAdd: initialize
			});

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const control = new (L.Control as any).TreeButton();
			control.addTo(m);

			button.set(control);
		}
	});
};
