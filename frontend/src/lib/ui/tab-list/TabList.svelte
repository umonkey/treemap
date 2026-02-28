<script lang="ts">
	import './TabList.css';

	const { items } = $props<{
		items: {
			title: string;
			link: string;
			active?: boolean;
		}[];
	}>();
</script>

<nav class="tab-list">
	<ul>
		{#each items as item}
			<li class:active={item.active}>
				<a href={item.link}>{item.title}</a>
			</li>
		{/each}
	</ul>
</nav>
