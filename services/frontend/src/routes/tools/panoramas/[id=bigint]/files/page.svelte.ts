import { getPanoramaFiles, type PanoramaFile } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'webp', 'gif', 'tif', 'tiff', 'bmp', 'heic'];

const isImage = (name: string): boolean => {
	const dot = name.lastIndexOf('.');
	if (dot === -1) {
		return false;
	}

	return IMAGE_EXTENSIONS.includes(name.slice(dot + 1).toLowerCase());
};

/** Non-image files (the processing log) first, then alphabetically. */
const compareFiles = (a: PanoramaFile, b: PanoramaFile): number => {
	const rank = (file: PanoramaFile) => (isImage(file.name) ? 1 : 0);
	const byRank = rank(a) - rank(b);

	return byRank !== 0 ? byRank : a.name.localeCompare(b.name);
};

export class PageState {
	files = $state<PanoramaFile[]>([]);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	sortedFiles = $derived.by(() => [...this.files].sort(compareFiles));

	reload = async (id: string) => {
		this.isLoading = true;
		this.error = undefined;
		const res = await getPanoramaFiles(id);
		if (res.status === 200 && res.data) {
			this.files = res.data;
		} else {
			this.files = [];
			this.error = res.error;
		}
		this.isLoading = false;
	};
}
