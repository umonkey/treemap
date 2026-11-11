import { exportPanorama } from '$lib/api/panoramas';
import { showError } from '$lib/errors';

export class PageState {
	isExporting = $state<boolean>(false);

	exportData = async (id: string) => {
		this.isExporting = true;
		const res = await exportPanorama(id);
		this.isExporting = false;
		if (res.status === 200 && res.data) {
			const jsonString = JSON.stringify(res.data, null, 2);
			const blob = new Blob([jsonString], { type: 'application/json' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `panorama-${id}.json`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
		} else if (res.error) {
			showError(res.error.description);
		}
	};
}
