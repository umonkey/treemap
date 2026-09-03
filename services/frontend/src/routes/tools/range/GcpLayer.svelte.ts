import type { IGcpWithRadius } from './MapPreview.svelte.ts';

export class GcpLayerLogic {
	isValidGcp = (gcp: IGcpWithRadius): boolean => {
		return (
			gcp != null &&
			!Number.isNaN(gcp.lat) &&
			!Number.isNaN(gcp.lng) &&
			!(gcp.lat === 0 && gcp.lng === 0)
		);
	};
}
