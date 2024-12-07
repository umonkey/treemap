<script type="ts">
    import { routes } from "$lib/routes";

    export let tree;
    export let active;

    const tabs = [
        {
            id: "details",
            title: "Details",
            url: routes.treeDetails(tree),
        },
        {
            id: "map",
            title: "Map",
            url: routes.treeMap(tree),
        },
        {
            id: "comments",
            title: "Comments",
            url: routes.treeComments(tree),
        },
        {
            id: "history",
            title: "History",
            url: routes.treeHistory(tree),
        },
    ];

    // In case tab id was not provided, default to the first tab.
    const activeTab = active ?? tabs[0].id;

    const cls = (tab) => {
        return tab == activeTab;
    };
</script>

<nav class="tabs">
    <ul>
        {#each tabs as tab}
            <li class:active={cls(tab.id)}><a href={tab.url}>{tab.title}</a></li>
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
