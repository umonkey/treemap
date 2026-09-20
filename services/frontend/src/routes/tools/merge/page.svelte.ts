import { getDuplicates } from '$lib/api/trees';
import type { DuplicateList, IError } from '$lib/types';
import { onPageFocus } from '$lib/utils/onPageFocus';

export class PageState {
	loading = $state<boolean>(true);
	data = $state<DuplicateList | undefined>(undefined);
	error = $state<IError | undefined>(undefined);

	reload = async (options?: { silent?: boolean }) => {
		if (!options?.silent) {
			this.loading = true;
		}

		try {
			const { status, data, error } = await getDuplicates();

			if (status === 200 && data) {
				this.data = data;
				this.error = undefined;
			} else {
				this.data = undefined;
				this.error = error;
			}
		} finally {
			if (!options?.silent) {
				this.loading = false;
			}
		}
	};

	setup = () => {
		return onPageFocus(() => {
			if (!this.loading) {
				this.reload({ silent: true });
			}
		});
	};
}
