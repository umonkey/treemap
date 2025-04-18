<script lang="ts">
	import { goto } from '$app/navigation';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { isMapperMode } from '$lib/stores/modeStore';
	import { AddTreeForm } from '$lib/forms';
	import type { ITree } from '$lib/types';

	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { Header } from '$lib/ui';

	const { data } = $props<{
		data: {
			lat: number;
			lng: number;
		};
	}>();

	const handleAdded = (tree: ITree) => {
		if ($isMapperMode) {
			goto(routes.mapPreview(tree.id));
		} else {
			goto(routes.treeDetails(tree.id));
		}
	};

	const handleCancel = () => {
		history.back();
	};
</script>

<svelte:head>
	<title>{locale.addTitle()}</title>
</svelte:head>

<Header title={locale.addTitle()} />

<div class="form">
	<AuthWrapper>
		<AddTreeForm lat={data.lat} lng={data.lng} onAdded={handleAdded} onCancel={handleCancel} />
	</AuthWrapper>
</div>

<style>
	.form {
		padding: 0 var(--gap) var(--gap);
	}
</style>
