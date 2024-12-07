<script type="ts">
    const tabs = [
        {
            id: "details",
            title: "Details",
            url: "/details",
        },
        {
            id: "map",
            title: "Map",
            url: "/map",
        },
        {
            id: "comments",
            title: "Comments",
            url: "/comments",
        },
        {
            id: "history",
            title: "History",
            url: "/history",
        },
    ];

    export let tree;
    export let active = tabs[0].id;

    const cls = (tab) => {
        return tab == active;
    };
</script>

<nav class="tabs">
    <ul>
        {#each tabs as tab}
            <li class:active={cls(tab.id)}><a href="#">{tab.title}</a></li>
        {/each}
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

            a {
                text-decoration: none;
                color: inherit;
            }
        }

        li.active {
            border-bottom-color: var(--tab-color);
            color: var(--text-color);
        }
    }
}
</style>
