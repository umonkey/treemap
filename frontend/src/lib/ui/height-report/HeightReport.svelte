<script lang="ts">
	type Props = {
		title: string;
		data: {
			value: number;
			count: number;
		}[];
	};

	const { data, title }: Props = $props();
</script>

{#if data}
	<h3>{title}</h3>

	<ul>
		{#each data as { value, count }}
			<li>
				{value} m: {count}
			</li>
		{/each}
	</ul>
{/if}
