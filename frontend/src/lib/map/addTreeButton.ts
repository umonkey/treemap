import { goto } from '$app/navigation';
import { routes } from '$lib/routes';
import L from 'leaflet';
import type { Map } from 'leaflet';

const round = (val: number): number => {
	const mul = 10000000;
	return Math.round(val * mul) / mul;
};

export const addTreeButton = (map: Map) => {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	(L.Control as any).TreeButton = L.Control.extend({
		options: {
			position: 'topleft'
		},

		onAdd: (map: Map) => {
			const container = L.DomUtil.create('div', 'leaflet-bar leaflet-control');

			const button = L.DomUtil.create('a', 'leaflet-tree-button', container);

			button.href = '#';
			button.type = 'button';

			const image = L.DomUtil.create('img', 'leaflet-control-button-icon', button);
			image.src = '/icons/tree.svg';
			image.width = 15;
			image.height = 30;

			L.DomEvent.disableClickPropagation(button);

			L.DomEvent.on(button, 'click', (e) => {
				e.preventDefault();
				const center = map.getCenter();
				goto(routes.treeAdd(round(center.lat), round(center.lng)));
			});

			return container;
		}
	});

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const control = new (L.Control as any).TreeButton();
	control.addTo(map);
};
