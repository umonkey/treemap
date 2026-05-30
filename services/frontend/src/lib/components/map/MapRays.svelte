<script lang="ts">
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
	import { mapRaysStore } from '$lib/stores/mapRays.svelte';

	const RAY_LENGTH = 100; // meters
	const R_EARTH = 6371000; // meters

	const calculateDestination = (lat: number, lng: number, angle: number) => {
		const brng = (angle * Math.PI) / 180;
		const lat1 = (lat * Math.PI) / 180;
		const lon1 = (lng * Math.PI) / 180;

		const lat2 = Math.asin(
			Math.sin(lat1) * Math.cos(RAY_LENGTH / R_EARTH) +
				Math.cos(lat1) * Math.sin(RAY_LENGTH / R_EARTH) * Math.cos(brng)
		);
		const lon2 =
			lon1 +
			Math.atan2(
				Math.sin(brng) * Math.sin(RAY_LENGTH / R_EARTH) * Math.cos(lat1),
				Math.cos(RAY_LENGTH / R_EARTH) - Math.sin(lat1) * Math.sin(lat2)
			);

		return [(lon2 * 180) / Math.PI, (lat2 * 180) / Math.PI];
	};

	const geojson = $derived({
		type: 'FeatureCollection' as const,
		features: mapRaysStore.rays.map((ray, index) => ({
			type: 'Feature' as const,
			id: index,
			geometry: {
				type: 'LineString' as const,
				coordinates: [[ray.lng, ray.lat], calculateDestination(ray.lat, ray.lng, ray.angle)]
			},
			properties: {}
		}))
	});
</script>

<GeoJSON data={geojson}>
	<LineLayer
		layout={{
			'line-cap': 'round',
			'line-join': 'round'
		}}
		paint={{
			'line-color': '#888',
			'line-width': 2,
			'line-dasharray': [2, 2]
		}}
	/>
</GeoJSON>
