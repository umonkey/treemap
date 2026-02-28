<script lang="ts">
	const { items } = $props<{
		items: {
			title: string;
			link: string;
			active?: boolean;
			badge?: string | number;
		}[];
	}>();
</script>

<nav class="tab-list">
	<ul>
		{#each items as item}
			<li class:active={item.active}>
				<a href={item.link}>
					{item.title}
					{#if item.badge}
						<span class="badge">{item.badge}</span>
					{/if}
				</a>
			</li>
		{/each}
	</ul>
</nav>

<style>
	.tab-list {
		border-bottom: solid 1px var(--sep-color);
		font-size: 14px;
		font-weight: 400;

		ul {
			list-style-type: none;
			margin: 0;
			padding: 0;
			display: flex;
			flex-direction: row;

			li {
				line-height: 30px;
				cursor: pointer;
				color: var(--text-color-inactive);
				padding: 0 calc(1.5 * var(--gap));

				a {
					display: flex;
					align-items: center;
					gap: 6px;
					text-decoration: none;
					color: inherit;
					border-bottom: 3px solid transparent;
				}

				&:hover {
					background-color: var(--tab-hover-color);
				}

				.badge {
					display: inline-block;
					background-color: #080;
					color: #fff;
					font-size: 11px;
					font-weight: 600;
					padding: 0 6px;
					border-radius: 5px;
					line-height: 16px;
					height: 16px;
					min-width: 16px;
					flex-shrink: 0;
					text-align: center;
				}
			}

			li.active {
				a {
					border-bottom-color: var(--tab-color);
					color: var(--text-color);
				}
			}
		}
	}
</style>
