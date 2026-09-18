import { getExportFiles } from '$lib/api/export';
import type { IError, IExportFile } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<IExportFile[]>([]);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;

			const { status, data: files, error: err } = await getExportFiles();

			if (status === 200 && files) {
				this.data = files;
				this.error = undefined;
			} else {
				this.data = [];
				this.error = err;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
