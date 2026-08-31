import { searchSpecies, suggestSpecies } from '$lib/api/species';
import type { IError, ISpecies } from '$lib/types';

export class SpeciesInputLogic {
	currentValue = $state<string>('');
	options = $state<ISpecies[]>([]);
	showOptions = $state<boolean>(false);
	suggested = $state<string[]>([]);
	loading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	formatValue = (v: string | null | undefined): string =>
		v === 'Unknown' || v === 'Unknown tree' ? '' : (v ?? '');

	syncValue = (v: string | null | undefined) => {
		this.currentValue = this.formatValue(v);
	};

	loadSuggested = async () => {
		try {
			this.loading = true;
			const { status, data, error } = await suggestSpecies();
			if (status === 200 && data) {
				this.suggested = data;
				this.error = undefined;
			} else {
				this.suggested = [];
				this.error = error;
			}
		} finally {
			this.loading = false;
		}
	};

	handleInput = (event: Event) => {
		const target = event.target as HTMLInputElement;

		searchSpecies(target.value).then((res) => {
			if (res.status === 200 && res.data) {
				this.options = res.data;
				this.showOptions = this.options.length > 0;
			}
		});
	};

	handleOptionClick = (e: Event, v: string, onChange: (value: string) => void) => {
		e.preventDefault();
		if (e.target instanceof HTMLElement) {
			e.target.blur();
		}

		this.showOptions = false;
		this.currentValue = this.formatValue(v);
		onChange(v);
	};

	handleSuggestionClick = (v: string, onChange: (value: string) => void) => {
		this.showOptions = false;
		this.currentValue = this.formatValue(v);
		onChange(v);
	};

	handleFocusOut = () => {
		setTimeout(() => {
			this.showOptions = false;
		}, 200);
	};

	handleChange = (e: Event, onChange: (value: string) => void) => {
		if (e.target) {
			const input = e.target as HTMLInputElement;
			onChange(input.value ?? '');
		}
	};
}
