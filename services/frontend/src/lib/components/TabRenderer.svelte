<script lang="ts">
	const {
		active,
		tabs
	}: {
		active?: string;
		tabs: {
			id: string;
			title: string;
			url: string;
			badge?: string | number;
		}[];
	} = $props();
</script>

<nav class="tabs">
	<ul>
		{#each tabs as tab}
			<li class:active={tab.id === active}>
				<a href={tab.url}>
					{tab.title}
					{#if tab.badge}
						<span class="badge">{tab.badge}</span>
					{/if}
				</a>
			</li>
		{/each}
	</ul>
</nav>

<style>
	.tabs {
		width: 100%;
		max-width: 100%;
		border-bottom: solid 1px var(--sep-color);
		font-size: 14px;
		font-weight: 400;

		ul {
			list-style-type: none;
			margin: 0;
			padding: 0;
			display: flex;
			flex-direction: row;
			overflow-x: auto;
			overflow-y: hidden;
			scrollbar-width: none;
			-ms-overflow-style: none;

			&::-webkit-scrollbar {
				display: none;
			}

			li {
				flex-shrink: 0;
				white-space: nowrap;
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

				.badge {
					display: inline-block;
					background-color: rgba(128, 128, 128, 0.2);
					color: rgba(128, 128, 128, 0.8);
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
