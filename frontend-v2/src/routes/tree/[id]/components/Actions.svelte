<script type="ts">
    import HeartIcon from '$lib/icons/HeartIcon.svelte';
    import ChatIcon from '$lib/icons/ChatIcon.svelte';
    import ShareIcon from '$lib/icons/ShareIcon.svelte';
    import SaveIcon from '$lib/icons/SaveIcon.svelte';
</script>

<div class="actions">
    <div><HeartIcon /></div>
    <div><ChatIcon /></div>
    <div>(map)</div>
    <div><ShareIcon /></div>
    <div><SaveIcon /></div>
</div>

<style>
.actions {
    display: flex;
    gap: var(--gap);
    justify-content: left;
    margin: var(--gap) 0;
}
</style>
