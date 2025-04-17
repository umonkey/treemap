<script lang="ts">
import { getAuthenticateUrl } from "$lib/osm";

const url = getAuthenticateUrl(window.location.origin);
window.location.href = url;
</script>
