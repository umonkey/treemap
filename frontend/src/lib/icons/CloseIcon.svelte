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
	<path
		d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>
