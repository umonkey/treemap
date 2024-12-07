<div class="gallery">
    <div class="imgno">2/10</div>
    <div class="slides">
        <div>
            <img src="https://yerevan.treemaps.app/v1/files/157235005582610432.jpg" />
            <div class="date">29.10.2019 &middot; umonkey You can just change the file extension to .svelte and import an SVG as a normal component.</div>
        </div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157234999588950016.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235134158999552.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235145743667200.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235424761352192.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235430486577152.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235418578948096.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235595687628800.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235721479000064.jpg" /></div>
        <div><img src="https://yerevan.treemaps.app/v1/files/157235709017722880.jpg" /></div>
    </div>
</div>

<style>
.gallery {
    position: relative;
}

.imgno {
    position: absolute;
    right: var(--gap);
    top: var(--gap);
    background-color: rgba(0, 0, 0, 0.85);
    color: #fff;
    border-radius: 6px;
    font-size: 14px;
    padding: 2px 4px;
    z-index: 2;
}

.slides {
    display: flex;
    max-width: 500px;
    max-height: 500px;

    overflow-x: auto;
    overflow-y: hidden;
    white-space: nowrap;
    scroll-snap-type: x mandatory;

    border-radius: 4px;

    & > div {
        display: block;
        position: relative;

        min-height: 100%;
        min-width: 100%;

        aspect-ratio: 1/1;
        text-align: center;
        align-content: center;
        scroll-snap-align: center;

        img {
            border: none;
            object-position: center;
            object-fit: cover;
            height: 100%;
            width: 100%;
        }
    }

    .date {
        position: absolute;
        bottom: var(--gap);
        left: var(--gap);
        background-color: rgba(0, 0, 0, 0.75);
        color: #fff;
        font-size: 14px;
        padding: 2px 4px;

        /* Prevent text overflow in case of really long username */
        max-width: calc(100% - 2 * var(--gap) - 8px);
        overflow: hidden;
        text-overflow: ellipsis;
    }
}
</style>
