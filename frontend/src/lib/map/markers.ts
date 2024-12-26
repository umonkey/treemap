/**
 * Wrap the very low-level logic of Leaflet markers.
 *
 * TODO:
 * - Change marker size based on zoom level, https://gis.stackexchange.com/questions/216558/leaflet-resize-markers-in-layer-when-zoom-in
 */

import L from 'leaflet';
import type { ITree } from '$lib/types';
import type { Map, Marker, LatLngBounds } from 'leaflet';
import { apiClient } from '$lib/api';

import BlackIcon from '$lib/map/icons/dot-black.svg';
import GreenIcon from '$lib/map/icons/dot-green.svg';
import RedIcon from '$lib/map/icons/dot-red.svg';
import YellorIcon from '$lib/map/icons/dot-yellow.svg';

type MarkerMap = {
	[key: string]: Marker;
};

type ClusterGroup = {
	lat: number;
	lon: number;
	count: number;

	// Additional props for panning.
	n: number;
	e: number;
	s: number;
	w: number;
};

type onChangeFn = (tree: ITree) => void;

export class Markers {
	private map;

	private markerMap: MarkerMap = {};
	private searchQuery: string | undefined = undefined;
	private bounds: LatLngBounds | undefined = undefined;

	private greenIcon;
	private yellowIcon;
	private redIcon;
	private blackIcon;

	public changeHandler: onChangeFn | null = null;

	private oldClusterGroups = [];

	constructor(map: Map, searchQuery: string | undefined) {
		this.map = map;
		this.searchQuery = searchQuery;

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

	public setSearchQuery(query: string | undefined) {
		this.searchQuery = query;
		this.reload();
	}

	public onChange(handler: onChangeFn) {
		this.changeHandler = handler;
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
			if (res.status == 200) {
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
		if (this.map.getZoom() < 18) {
			return this.getClusterGroupsToShow(trees);
		} else {
			return this.getMarkersToShow(trees);
		}
	}

	private getMarkersToShow(trees: ITree[]) {
		const res = [];

		for (const tree of trees) {
			const point = L.marker([tree.lat, tree.lon], {
				icon: this.getTreeIcon(tree)
			});

			point.on('click', () => {
				this.map.panTo([tree.lat, tree.lon]);

				if (this.changeHandler) {
					this.changeHandler(tree);
				}
			});

			res.push(point);
		}

		return res;
	}

	private getClusterGroupsToShow(trees: ITree[]) {
		const res = [];

		const r = this.getClusterGroupRadius();

		for (const group of this.splitBuckets(trees)) {
			const onClick = () => {
				this.map.fitBounds([
					[group.s, group.w],
					[group.n, group.e]
				]);
			};

			const circle = L.circleMarker([group.lat, group.lon], {
				color: '#080',
				fillColor: '#080',
				fillOpacity: 0.5,
				radius: r
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
		const bounds = this.map.getBounds();
		const n = bounds.getNorth();
		const e = bounds.getEast();
		const s = bounds.getSouth();
		const w = bounds.getWest();

		const step_x = (e - w) / 10;
		const step_y = (n - s) / 10;

		const buckets = {};

		for (const tree of trees) {
			if (tree.lat > n || tree.lat < s) {
				continue;
			}

			if (tree.lon > e || tree.lon < w) {
				continue;
			}

			const y = Math.floor((tree.lat - s) / step_y);
			const x = Math.floor((tree.lon - w) / step_x);

			const id = `${x}-${y}`;

			const bucket = buckets[id] || {
				lat: s + y * step_y + step_y / 2,
				lon: w + x * step_x + step_x / 2,
				count: 0,

				n: s + y * step_y + step_y,
				e: w + x * step_x + step_x,
				s: s + y * step_y,
				w: w + x * step_x
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
