<nav class="tabs">
    <ul>
        <li class="active">Details</li>
        <li>Map</li>
        <li>Comments</li>
        <li>History</li>
    </ul>
</nav>

<style>
.tabs {
    padding: 0 var(--gap);
    border-bottom: solid 1px var(--sep-color);

    ul {
        list-style-type: none;
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: row;
        gap: calc(2 * var(--gap));

        li {
            border-bottom: 3px solid transparent;
            line-height: 40px;
            cursor: pointer;
            color: var(--text-color-inactive);
            font-weight: 600;
        }

        li.active {
            border-bottom-color: var(--tab-color);
            color: var(--text-color);
        }
    }
}
</style>
