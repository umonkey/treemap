import { mergeDuplicates } from '$lib/api/trees';
import { routes, goto } from '$lib/routes';
import type { IError } from '$lib/types';

export class PageState {
	src = $state<string>('');
	dst = $state<string>('');
	confirmed = $state<boolean>(false);
	saving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	init = (src: string, dst: string) => {
		this.src = src;
		this.dst = dst;
	};

	setSrc = (value: string) => {
		this.src = value;
	};

	setDst = (value: string) => {
		this.dst = value;
	};

	canSubmit = $derived(
		this.src.trim() !== '' &&
			this.dst.trim() !== '' &&
			this.src.trim() !== this.dst.trim() &&
			this.confirmed &&
			!this.saving
	);

	canSwap = $derived(/^\d+$/.test(this.src.trim()) && /^\d+$/.test(this.dst.trim()));

	swap = () => {
		if (!this.canSwap) {
			return;
		}

		const nextSrc = this.dst.trim();
		const nextDst = this.src.trim();

		// Update the form immediately so it stays in sync even when the swapped
		// URL is identical to the current one (e.g. after manual edits).
		this.src = nextSrc;
		this.dst = nextDst;

		goto(routes.toolsMergePair(nextSrc, nextDst));
	};

	submit = async () => {
		if (!this.canSubmit) {
			return;
		}

		this.saving = true;
		this.error = undefined;

		const res = await mergeDuplicates(this.src.trim(), this.dst.trim());

		this.saving = false;

		if (res.status === 204) {
			goto(routes.toolsMerge());
		} else {
			this.error = res.error;
		}
	};

	cancel = () => {
		goto(routes.toolsMerge());
	};
}
