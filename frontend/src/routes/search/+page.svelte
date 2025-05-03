<script lang="ts">
	import { locale } from '$lib/locale';
	import { SearchForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';
</script>

<NarrowPage title={locale.searchTitle()}>
	<SearchForm />
</NarrowPage>
