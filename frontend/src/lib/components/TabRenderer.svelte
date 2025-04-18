<script lang="ts">
	const { active, tabs } = $props<{
		active: string;
		tabs: {
			id: string;
			title: string;
			url: string;
		}[];
	}>();

	// In case tab id was not provided, default to the first tab.
	const activeTab = active ?? tabs[0].id;

	const cls = (tab: string) => {
		return tab === activeTab;
	};
</script>

<nav class="tabs">
	<ul>
		{#each tabs as tab}
			<li class:active={cls(tab.id)}><a href={tab.url}>{tab.title}</a></li>
		{/each}
	</ul>
</nav>

<style>
	.tabs {
		border-bottom: solid 1px var(--sep-color);
		font-size: 16px;

		ul {
			list-style-type: none;
			margin: 0;
			padding: 0;
			display: flex;
			flex-direction: row;

			li {
				line-height: 40px;
				cursor: pointer;
				color: var(--text-color-inactive);
				font-weight: 600;
				padding: 0 calc(1.5 * var(--gap));

				a {
					display: block;
					text-decoration: none;
					color: inherit;
					border-bottom: 3px solid transparent;
				}

				&:hover {
					background-color: var(--tab-hover-color);
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
