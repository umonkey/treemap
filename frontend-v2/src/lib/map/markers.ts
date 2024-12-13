/**
 * Wrap the very low-level logic of Leaflet markers.
 *
 * TODO:
 * - Change marker size based on zoom level, https://gis.stackexchange.com/questions/216558/leaflet-resize-markers-in-layer-when-zoom-in
 */

import L from 'leaflet';
import type { ITree } from '$lib/types';
import type { Map, Marker } from 'leaflet';
import { apiClient } from '$lib/api';

import BlackIcon from '$lib/map/icons/dot-black.svg';
import GreenIcon from '$lib/map/icons/dot-green.svg';
import RedIcon from '$lib/map/icons/dot-red.svg';
import YellorIcon from '$lib/map/icons/dot-yellow.svg';

type MarkerMap = {
	[key: string]: Marker;
};

type onChangeFn = (tree: ITree) => void;

export class Markers {
	private map;

	private markerMap: MarkerMap = {};

	private greenIcon;
	private yellowIcon;
	private redIcon;
	private blackIcon;

	public changeHandler: onChangeFn | null = null;

	constructor(map: Map) {
		this.map = map;

		this.greenIcon = L.icon({
			iconUrl: GreenIcon,
			iconSize: [20, 20],
			iconAnchor: [10, 10]
		});

		this.yellowIcon = L.icon({
			iconUrl: YellorIcon,
			iconSize: [20, 20],
			iconAnchor: [10, 10]
		});

		this.redIcon = L.icon({
			iconUrl: RedIcon,
			iconSize: [20, 20],
			iconAnchor: [10, 10]
		});

		this.blackIcon = L.icon({
			iconUrl: BlackIcon,
			iconSize: [20, 20],
			iconAnchor: [10, 10]
		});

		map.on('moveend', () => this.onMoveEnd());

		this.onMoveEnd();
	}

	public onChange(handler: onChangeFn) {
		this.changeHandler = handler;
	}

	private async onMoveEnd() {
		const bounds = this.map.getBounds();
		const n = bounds.getNorth();
		const e = bounds.getEast();
		const s = bounds.getSouth();
		const w = bounds.getWest();

		const res = await apiClient.getMarkers(n, e, s, w);

		if (res.status === 200) {
			this.replaceMarkers(res.data.trees);
		}
	}

	/**
	 * Replaces the current markers with the given markers.
	 *
	 * Leaflet cannot track duplicates so we have to do this on our side.
	 */
	private replaceMarkers(trees: ITree[]) {
		const oldKeys = Object.keys(this.markerMap);
		const newKeys = trees.map((m) => m.id);

		// Add new markers.
		for (const tree of trees) {
			if (!oldKeys.includes(tree.id)) {
				const point = L.marker([tree.lat, tree.lon], {
					icon: this.getTreeIcon(tree)
				});

				point.addTo(this.map).on('click', () => {
					this.map.panTo([tree.lat, tree.lon]);

					if (this.changeHandler) {
						this.changeHandler(tree);
					}
				});

				this.markerMap[tree.id] = point;
				oldKeys.push(tree.id);
			}
		}

		// Remove gone markers.
		for (const key of oldKeys) {
			if (!newKeys.includes(key)) {
				this.markerMap[key].remove();
				delete this.markerMap[key];
			}
		}
	}

	private getTreeIcon(tree: ITree) {
		if (tree.state === 'dead' || tree.state === 'gone' || tree.state === 'stomp') {
			return this.blackIcon;
		}

		if (tree.state === 'sick') {
			return this.redIcon;
		}

		if (tree.state === 'deformed') {
			return this.yellowIcon;
		}

		return this.greenIcon;
	}
}
