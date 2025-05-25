<script lang="ts">
	import { locale } from '$lib/locale';
	import { getTree, treeStore } from '$lib/stores/treeStore';
	import type { ITree } from '$lib/types';
	import { formatCentimeters, formatState } from '$lib/utils/trees';
	import { CircumferenceEditor, NarrowPage, Form, StateEditor, AuthWrapper } from '$lib/ui';
	import { CircumferenceIcon, EditIcon, HelpIcon, StateIcon } from '$lib/icons';

	const { data } = $props();
	let tree = $state<ITree>($getTree(data.treeId));

	let tab = $state<string | null>(null);

	const onClose = () => {
		tab = null;
	};

	// WTF is this needed? Why doesn't it work if we just reference the store directly?
	treeStore.subscribe((trees) => {
		tree = trees[data.treeId];
	});

	const setTab = (value: string) => {
		tab = value;
	};
</script>

<NarrowPage title={locale.measureTitle()}>
	<AuthWrapper>
		{#if tab === 'circumference'}
			<CircumferenceEditor {tree} {onClose} />
		{:else if tab === 'state'}
			<StateEditor {tree} {onClose} />
		{:else}
			<Form>
				<div class="row">
					<CircumferenceIcon />
					<span class="label">{locale.propTrunk()}: {formatCentimeters(tree.circumference)}</span>
					<button type="button" onclick={() => setTab('circumference')}><EditIcon /></button>
					<a href="https://myga.am/app/measuring-circumference.html" target="_blank"><HelpIcon /></a
					>
				</div>

				<div class="row">
					<StateIcon />
					<span class="label">{locale.propState()}: {formatState(tree.state)}</span>
					<button type="button" onclick={() => setTab('state')}><EditIcon /></button>
					<a href="https://myga.am/app/measuring-state.html" target="_blank"><HelpIcon /></a>
				</div>
			</Form>
		{/if}
	</AuthWrapper>
</NarrowPage>

<style>
	button {
		background-color: transparent;
		border: none;
		cursor: pointer;
		padding: 0;
		margin: 0;
		color: inherit;
		display: block;
	}
</style>
