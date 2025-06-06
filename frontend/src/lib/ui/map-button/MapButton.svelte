<script lang="ts">
	import { hooks } from './hooks';
	import { onMount } from 'svelte';
	import './styles.css';

	const {
		position = 'topleft',
		onClick,
		icon,
		active = false
	} = $props<{
		position: string;
		icon: string;
		active?: boolean;
		onClick: () => void;
	}>();

	const { handleImageChange, handlePositionChange, handleActiveChange } = hooks({
		onMount,
		onClick,
		icon,
		position
	});

	$effect(() => handleImageChange(icon));
	$effect(() => handlePositionChange(position));
	$effect(() => handleActiveChange(!!active));
</script>
