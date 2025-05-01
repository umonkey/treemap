// Wrap the very low-level logic of Leaflet markers.
//
// TODO:
// - Change marker size based on zoom level, https://gis.stackexchange.com/questions/216558/leaflet-resize-markers-in-layer-when-zoom-in

import { apiClient } from '$lib/api';
import type { ITree } from '$lib/types';
import L from 'leaflet';
import type { LatLngBounds, Map, Marker } from 'leaflet';
import { mapBus } from '$lib/buses';

import BlackIcon from '$lib/map/icons/dot-black.svg';
import GreenIcon from '$lib/map/icons/dot-green.svg';
import RedIcon from '$lib/map/icons/dot-red.svg';
import YellowIcon from '$lib/map/icons/dot-yellow.svg';

// Only start clustering when showing this number of trees.
const MIN_CLUSTER_SIZE = 200;

type MarkerMap = {
	[key: string]: Marker;
};

const CLUSTER_GRID: {
	[key: number]: number;
} = {
	1: 26.214,
	2: 13.107,
	3: 6.5535,
	4: 3.2768,
	5: 1.6384,
	6: 0.8192,
	7: 0.4096,
	8: 0.2048,
	9: 0.1024,
	10: 0.0512,
	11: 0.0256,
	12: 0.0128,
	13: 0.0064,
	14: 0.0032,
	15: 0.0016,
	16: 0.0008,
	17: 0.0004,
	18: 0.0001220703125
};

const CLUSTER_RADIUS: {
	[key: number]: number;
} = {
	6: 32000,
	7: 16000,
	8: 8000,
	9: 4000,
	10: 2000,
	11: 1000,
	12: 500,
	13: 250,
	14: 125,
	15: 62.5,
	16: 31.25,
	17: 15.625
};

type ClusterGroup = {
	lat: number;
	lon: number;
	radius: number;
	count: number;

	// Additional props for panning.
	n: number;
	e: number;
	s: number;
	w: number;
};

export class Markers {
	private map;

	private markerMap: MarkerMap = {};
	private searchQuery: string | undefined = undefined;
	private bounds: LatLngBounds | undefined = undefined;

	private greenIcon;
	private yellowIcon;
	private redIcon;
	private blackIcon;

	private oldClusterGroups: L.Layer[] = [];

	constructor(map: Map, searchQuery: string | undefined) {
		this.map = map;
		this.searchQuery = searchQuery;

		this.greenIcon = L.icon({
			iconUrl: GreenIcon,
			iconSize: [20, 20],
			iconAnchor: [10, 10]
		});

		this.yellowIcon = L.icon({
			iconUrl: YellowIcon,
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

	public setSearchQuery(query: string | undefined) {
		this.searchQuery = query;
		this.reload();
	}

	private async onMoveEnd() {
		this.bounds = this.map.getBounds();
		this.reload();
	}

	/**
	 * Reload markers after a change in parameters.
	 */
	private reload() {
		if (!this.bounds) {
			console.debug('[map] Not reloading -- bounds not set.');
			return;
		}

		const n = this.bounds.getNorth();
		const e = this.bounds.getEast();
		const s = this.bounds.getSouth();
		const w = this.bounds.getWest();

		apiClient.getMarkers(n, e, s, w, this.searchQuery).then((res) => {
			if (res.status === 200 && res.data) {
				const trees = res.data.trees;
				console.debug(`[map] Received ${trees.length} trees, search=${this.searchQuery}.`);
				this.replaceMarkers(trees);
			}
		});
	}

	/**
	 * Replaces the current markers with the given markers.
	 *
	 * Leaflet cannot track duplicates so we have to do this on our side.
	 */
	private replaceMarkers(trees: ITree[]) {
		const items = this.getItemsToShow(trees);

		// Remove old items.
		for (const item of this.oldClusterGroups) {
			this.map.removeLayer(item);
		}

		// Add new items.
		for (const item of items) {
			this.map.addLayer(item);
			this.oldClusterGroups.push(item);
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

	/**
	 * Returns individual trees or cluster groups, depending
	 * on the map zoom level.
	 */
	private getItemsToShow(trees: ITree[]) {
		if (this.map.getZoom() < 18 && trees.length >= MIN_CLUSTER_SIZE) {
			return this.getClusterGroupsToShow(trees);
		}

		return this.getMarkersToShow(trees);
	}

	private getMarkersToShow(trees: ITree[]) {
		const res = [];

		for (const tree of trees) {
			const point = L.marker([tree.lat, tree.lon], {
				icon: this.getTreeIcon(tree)
			});

			point.on('click', () => {
				mapBus.emit('select', tree.id);
			});

			res.push(point);
		}

		return res;
	}

	private getClusterGroupsToShow(trees: ITree[]) {
		const res = [];

		for (const group of this.splitBuckets(trees)) {
			const onClick = () => {
				this.map.fitBounds([
					[group.s, group.w],
					[group.n, group.e]
				]);
			};

			const circle = L.circle([group.lat, group.lon], {
				color: '#080',
				fillColor: '#080',
				fillOpacity: 0.5,
				radius: group.radius
			}).on('click', onClick);

			const label = L.divIcon({
				html: group.count.toString(),
				className: 'cluster-count',
				iconSize: [40, 40]
			});

			const marker = L.marker([group.lat, group.lon], {
				icon: label
			}).on('click', onClick);

			res.push(circle);
			res.push(marker);
		}

		return res;
	}

	/**
	 * Split trees into 100 separate buckets (clustering).
	 */
	private splitBuckets(trees: ITree[]): ClusterGroup[] {
		const divider = CLUSTER_GRID[this.map.getZoom()];

		const buckets: {
			[key: string]: ClusterGroup;
		} = {};

		const radius = CLUSTER_RADIUS[this.map.getZoom()] ?? 100;

		for (const tree of trees) {
			const y = Math.round(tree.lat / divider) * divider;
			const x = Math.round(tree.lon / divider) * divider;

			const id = `${x},${y}`;

			const bucket = buckets[id] || {
				lat: y,
				lon: x,
				count: 0,
				radius,

				n: y + divider / 2,
				e: x + divider / 2,
				s: y - divider / 2,
				w: x - divider / 2
			};

			bucket.count++;

			buckets[id] = bucket;
		}

		return Object.values(buckets);
	}

	private getClusterGroupRadius(): number {
		const container = this.map.getContainer();

		const width = container.clientWidth;
		const height = container.clientHeight;

		const min = Math.min(width, height);

		return min / 10 / 2 - 5;
	}
}
