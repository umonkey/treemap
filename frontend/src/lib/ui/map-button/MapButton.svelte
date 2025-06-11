<script lang="ts">
	import { hooks } from './hooks';
	import { onMount } from 'svelte';
	import './styles.css';

	const {
		position = 'topleft',
		onClick,
		icon,
		active = false,
		disabled = false
	} = $props<{
		position: string;
		icon: string;
		active?: boolean;
		disabled?: boolean;
		onClick: () => void;
	}>();

	const { handleImageChange, handlePositionChange, handleActiveChange, handleDisabledChange } =
		hooks({
			onMount,
			onClick,
			icon,
			position
		});

	$effect(() => handleImageChange(icon));
	$effect(() => handlePositionChange(position));
	$effect(() => handleActiveChange(!!active));
	$effect(() => handleDisabledChange(!!disabled));
</script>
