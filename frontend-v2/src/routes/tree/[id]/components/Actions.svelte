<script type="ts">
    import HeartIcon from '$lib/icons/HeartIcon.svelte';
    import ChatIcon from '$lib/icons/ChatIcon.svelte';
    import ShareIcon from '$lib/icons/ShareIcon.svelte';
    import SaveIcon from '$lib/icons/SaveIcon.svelte';
    import MapIcon from '$lib/icons/MapIcon.svelte';
</script>

<div class="actions">
    <div><HeartIcon class="icon" /></div>
    <div><ChatIcon class="icon" /></div>
    <div><MapIcon class="icon" /></div>
    <div><ShareIcon class="icon" /></div>
    <div class="sep"></div>
    <div><SaveIcon class="icon" /></div>
</div>

<style>
.actions {
    display: flex;
    gap: calc(var(--gap) * 1.5);
    justify-content: left;
    margin: var(--gap) 0;
    padding: 0 var(--gap);

    & > div {
        width: 24px;
        height: 24px;
        color: var(--color-text);
        text-align: center;
    }

    .sep {
        flex-grow: 1;
    }
}
</style>
