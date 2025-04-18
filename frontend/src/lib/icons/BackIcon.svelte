<script lang="ts">
	const { width = '100%', height = '100%' } = $props<{
		width: string;
		height: string;
	}>();
</script>

<svg
	{width}
	{height}
	data-slot="icon"
	aria-hidden="true"
	fill="none"
	stroke-width="1.5"
	stroke="currentColor"
	viewBox="0 0 24 24"
	xmlns="http://www.w3.org/2000/svg"
>
	<path d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" stroke-linecap="round" stroke-linejoin="round"
	></path>
</svg>
