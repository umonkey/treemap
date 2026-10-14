import { getTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { treeStore } from '$lib/stores/treeStore';
import type { IError, ISingleTree } from '$lib/types';
import { fromStore } from 'svelte/store';

export const TREE_CONTEXT_KEY = Symbol('tree-layout');

export class TreeLayoutState {
	id = $state<string>();

	// The single-tree payload, kept for fields the map cache doesn't carry
	// (e.g. comment_count / users).
	response = $state<ISingleTree>();

	#trees = fromStore(treeStore);

	/** Cache is the source of truth; single-tree-only fields survive the merge. */
	tree = $derived.by<ISingleTree | undefined>(() => {
		const cached = this.id ? this.#trees.current[this.id] : undefined;

		if (!cached && !this.response) {
			return undefined;
		}

		return { ...this.response, ...cached } as ISingleTree;
	});

	loading = $state(false);
	error = $state<IError>();
	expand = $state(false);
	query = $state<string>('');

	public reload = (id: string) => {
		this.id = id;
		this.error = undefined;
		this.loading = true;

		getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.response = res.data;

				const ll = { lat: res.data.lat, lng: res.data.lon };
				mapBus.emit('pin', ll);
				mapBus.emit('moveOnce', ll);
			} else if (res.error) {
				this.error = res.error;
				showError(res.error.description);
			}
			this.loading = false;
		});
	};

	public toggleExpand = () => {
		this.expand = !this.expand;
	};

	public copyId = async () => {
		if (!this.tree) {
			return;
		}

		try {
			await navigator.clipboard.writeText(this.tree.id);
		} catch (e) {
			console.error('Failed to copy tree id to clipboard', e);
		}
	};

	public handleInput = (value: string) => {
		this.query = value;
	};

	public handleSearch = async (value: string) => {
		await goto(routes.searchQuery(value));
	};

	public handleClose = async (isRoot: boolean) => {
		if (!this.id) {
			return;
		}

		await goto(isRoot ? routes.home() : routes.treeDetails(this.id));
	};

	/** Cleanup leaves the map pin behind. */
	public init = () => {
		return () => {
			mapBus.emit('pin', undefined);
		};
	};
}
