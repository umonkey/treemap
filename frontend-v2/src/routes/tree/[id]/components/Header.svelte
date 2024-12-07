<script>
    import BackIcon from '$lib/icons/BackIcon.svelte';
</script>

<div class="header">
    <div class="icon"><div><BackIcon /></div></div>
    <div>Tree</div>
    <div class="icon"></div>
</div>

<style>
.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--gap);
    border-bottom: 1px solid var(--sep-color);
    font-weight: 800;

    .icon {
        flex-basis: 30px;
        flex-grow: 0;
        flex-shrink: 0;
        height: 30px;
        align-content: center;

        & > div {
            width: 20px;
            height: 20px;
            padding: 5px;
            cursor: pointer;
        }
    }
}
</style>
