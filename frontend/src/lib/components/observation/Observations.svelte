<script lang="ts">
	import { locale } from '$lib/locale';
	import type { IObservation } from '$lib/types';

	const { observation } = $props<{
		observation: IObservation | null;
	}>();

	const fields = [
		{ id: 'bark_damage', label: locale.observationBarkDamage(), beneficial: false },
		{ id: 'dry_branches', label: locale.observationDryBranches(), beneficial: false },
		{ id: 'leaking', label: locale.observationLeaking(), beneficial: false },
		{ id: 'root_damage', label: locale.observationRootDamage(), beneficial: false },
		{ id: 'open_roots', label: locale.observationOpenRoots(), beneficial: false },
		{ id: 'bug_holes', label: locale.observationBugHoles(), beneficial: false },
		{ id: 'topping', label: locale.observationTopping(), beneficial: false },
		{ id: 'fungal_bodies', label: locale.observationFungalBodies(), beneficial: false },
		{ id: 'vfork', label: locale.observationVfork(), beneficial: false },
		{ id: 'cavities', label: locale.observationCavities(), beneficial: false },
		{ id: 'vines', label: locale.observationVines(), beneficial: false },
		{ id: 'inclined', label: locale.observationInclined(), beneficial: false },
		{ id: 'nests', label: locale.observationNests(), beneficial: true },
		{ id: 'nesting_boxes', label: locale.observationNestingBoxes(), beneficial: true }
	];

	const active = $derived(
		observation
			? fields.filter((field) => observation[field.id as keyof IObservation] === true)
			: []
	);
</script>

{#if active.length > 0}
	<div class="observations">
		<ul>
			{#each active as field}
				<li class:beneficial={field.beneficial}>
					<span class="mark">{field.beneficial ? '✓' : '✗'}</span>
					{field.label}
				</li>
			{/each}
		</ul>
	</div>
{/if}

<style>
	.observations {
		padding: 0 var(--gap);
		margin-bottom: var(--gap);

		ul {
			list-style: none;
			padding: 0;
			margin: 0;
			display: flex;
			flex-wrap: wrap;
			gap: 5px;

			li {
				background-color: var(--form-background);
				padding: 2px 8px;
				border-radius: 4px;
				font-size: 0.9em;
				display: flex;
				align-items: center;
				gap: 4px;

				.mark {
					font-weight: bold;
					color: #ef5554; /* defect color */
				}

				&.beneficial .mark {
					color: #4cd964; /* beneficial color */
				}
			}
		}
	}
</style>
