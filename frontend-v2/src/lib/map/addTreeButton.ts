import { goto } from '$app/navigation';
import L from 'leaflet';
import { Map } from 'leaflet';
import { routes } from '$lib/routes';

export const addTreeButton = (map: Map) => {
	L.Control.TreeButton = L.Control.extend({
		options: {
			position: 'topleft'
		},

		onAdd: function (map: Map) {
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
				goto(routes.treeAdd(center.lat, center.lng));
			});

			return container;
		}
	});

	const control = new L.Control.TreeButton();
	control.addTo(map);
};
