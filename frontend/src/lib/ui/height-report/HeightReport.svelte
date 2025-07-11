<script lang="ts">
	type Props = {
		data: {
			value: number;
			count: number;
		}[];
	};

	const { data }: Props = $props();
</script>

{#if data}
	<h3>Trees by height</h3>

	<ul>
		{#each data as { value, count }}
			<li>
				{value} m: {count}
			</li>
		{/each}
	</ul>
{/if}
