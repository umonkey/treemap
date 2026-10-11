<script lang="ts">
	import BaseContextMenu from '$lib/components/menu/BaseContextMenu.svelte';
	import type { IContextMenuItem } from '$lib/components/menu/BaseContextMenu.svelte.ts';
	import DownloadIcon from '$lib/icons/DownloadIcon.svelte';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import MapTreeIcon from '$lib/icons/MapTreeIcon.svelte';
	import Ruler from '$lib/icons/Ruler.svelte';
	import WaterIcon from '$lib/icons/WaterIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { hasPermission } from '$lib/stores/authStore';
	import { ContextMenuLogic } from './ContextMenu.svelte.ts';

	const componentState = new ContextMenuLogic();

	$effect(() => componentState.subscribe());

	const items = $derived<IContextMenuItem[]>([
		{
			icon: MapTreeIcon,
			label: locale.addTitle(),
			disabled: !$hasPermission('tree:create'),
			onClick: () => componentState.handleNavigate(routes.addTree())
		},
		{
			icon: Ruler,
			label: locale.addRowTitle(),
			disabled: !$hasPermission('tree:create'),
			onClick: () => componentState.handleNavigate(routes.addRow())
		},
		{
			icon: WaterIcon,
			label: locale.waterAddTitle(),
			disabled: !$hasPermission('water:manage'),
			onClick: () => componentState.handleNavigate(routes.addWater())
		},
		{
			icon: DownloadIcon,
			label: locale.downloadData(),
			href: componentState.downloadUrl,
			download: true,
			separator: true,
			onClick: componentState.handleDownload
		},
		{
			icon: LocationIcon,
			label: locale.copyCoordinates(),
			onClick: componentState.handleCopyCoordinates
		}
	]);
</script>

<BaseContextMenu
	open={componentState.open}
	position={componentState.position}
	{items}
	onClose={componentState.handleClose}
/>
