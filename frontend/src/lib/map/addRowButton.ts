// Adds a button to add a row of trees.
//
// The button has a ruler icon.  When clicked, stores the original position,
// and allows the user to select a second point.  Draws a point between the
// two points, and then redirects to the tree add page with the coordinates.
//
// TODO: remove moveend handler on destroy.
// TODO: change button color when active.
// TODO: reset on second button click.
// TODO: display confirmation button.
// TODO: display distance between points.

import L from 'leaflet';
import type { Map } from 'leaflet';
import RULER from '$lib/icons/RulerIcon.svg';
import { get, writable } from 'svelte/store';
import type { ILatLng } from '$lib/types';

export const addRowButton = (map: Map) => {
	const first = writable<ILatLng | null>(null);
	const second = writable<ILatLng | null>(null);
	const line = writable<L.Polyline | null>(null);

	second.subscribe((p2: ILatLng | null) => {
		const p1 = get(first);

		if (p1 === null || p2 === null) {
			console.debug('[addRowButton] No first point set, not drawing line.');
			return;
		}

		console.debug(`[addRowButton] drawing line`);

		get(line)?.remove();
		line.set(null);

		const l = L.polyline([p1, p2], {
			color: 'blue',
			weight: 2,
			opacity: 0.5
		});

		l.addTo(map);

		line.set(l);
	});

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	(L.Control as any).RowButton = L.Control.extend({
		options: {
			position: 'topleft'
		},

		onAdd: (map: Map) => {
			const container = L.DomUtil.create('div', 'leaflet-bar leaflet-control');

			const button = L.DomUtil.create('a', 'leaflet-tree-button', container);

			button.href = '#';
			button.type = 'button';

			const image = L.DomUtil.create('img', 'leaflet-control-button-icon', button);
			image.src = RULER;
			image.width = 15;
			image.height = 30;

			L.DomEvent.disableClickPropagation(button);

			// Clicking the button starts the process of adding a row of trees.
			L.DomEvent.on(button, 'click', (e) => {
				e.preventDefault();

				const center = map.getCenter();

				first.set({
					lat: center.lat,
					lng: center.lng
				});

				console.debug(`[addRowButton] Row start updated: ${center.lat},${center.lng}.`);
			});

			// When the map moves, update the end of the row.
			map.on('moveend', () => {
				if (get(first) === null) {
					console.debug('[addRowButton] Map moved, but row not started.');
					return;
				}

				const center = map.getCenter();

				second.set({
					lat: center.lat,
					lng: center.lng
				});

				console.debug(`[addRowButton] Row end updated: ${center.lat},${center.lng}.`);
			});

			return container;
		}
	});

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const control = new (L.Control as any).RowButton();
	control.addTo(map);
};
