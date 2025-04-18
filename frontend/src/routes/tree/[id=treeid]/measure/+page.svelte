<script lang="ts">
	import { locale } from '$lib/locale';
	import { getTree, treeStore } from '$lib/stores/treeStore';
	import type { ITree } from '$lib/types';
	import { formatCentimeters, formatMeters, formatState } from '$lib/utils/trees';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { CanopyEditor, CircumferenceEditor, Header, HeightEditor, StateEditor } from '$lib/ui';
	import {
		CircumferenceIcon,
		DiameterIcon,
		EditIcon,
		HeightIcon,
		HelpIcon,
		StateIcon
	} from '$lib/icons';

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

<svelte:head>
	<title>{locale.measureTitle()}</title>
</svelte:head>

<Header title={locale.measureTitle()} />

<div class="padded measure">
	<AuthWrapper>
		{#if tab === 'height'}
			<HeightEditor {tree} {onClose} />
		{:else if tab === 'canopy'}
			<CanopyEditor {tree} {onClose} />
		{:else if tab === 'circumference'}
			<CircumferenceEditor {tree} {onClose} />
		{:else if tab === 'state'}
			<StateEditor {tree} {onClose} />
		{:else}
			<div class="form">
				<div class="row">
					<HeightIcon />
					<span class="label">{locale.propHeight()}: {formatMeters(tree.height)}</span>
					<button type="button" onclick={() => setTab('height')}><EditIcon /></button>
					<a href="https://myga.am/app/measuring-height.html" target="_blank"><HelpIcon /></a>
				</div>
				<div class="row">
					<DiameterIcon />
					<span class="label">{locale.propCanopy()}: {formatMeters(tree.diameter)}</span>
					<button type="button" onclick={() => setTab('canopy')}><EditIcon /></button>
					<a href="https://myga.am/app/measuring-canopy.html" target="_blank"><HelpIcon /></a>
				</div>
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
			</div>
		{/if}
	</AuthWrapper>
</div>

<style>
	.form {
		line-height: 30px;
		padding-top: var(--gap);
		display: flex;
		flex-direction: column;
		gap: var(--gap);

		.row {
			display: flex;
			flex-direction: row;
			gap: var(--gap);
			color: var(--text-color);
			align-items: center;
		}

		.label {
			flex-grow: 1;
			flex-shrink: 0;
		}

		a {
			color: inherit;
			display: block;
			line-height: 0;
		}

		button {
			background-color: transparent;
			border: none;
			cursor: pointer;
			padding: 0;
			margin: 0;
			color: inherit;
			display: block;
		}

		:global(svg) {
			width: 30px;
			height: 30px;
		}
	}
</style>
