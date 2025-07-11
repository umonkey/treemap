<script lang="ts">
	type Props = {
		data: {
			state: string;
			count: number;
		}[];
	};

	const { data }: Props = $props();
</script>

{#if data}
	<h3>Trees by state</h3>

	<ul>
		{#each data as { state, count }}
			<li>
				{state}: {count}
			</li>
		{/each}
	</ul>
{/if}
