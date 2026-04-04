import { getDuplicates } from '$lib/api/trees';
import type { DuplicateList, IError } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<DuplicateList | undefined>(undefined);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;
			const { status, data, error } = await getDuplicates();

			if (status === 200 && data) {
				this.data = data;
				this.error = undefined;
			} else {
				this.data = undefined;
				this.error = error;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
