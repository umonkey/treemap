<div class="images">
    <div><img src="https://yerevan.treemaps.app/v1/files/157235005582610432.jpg" /></div>
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

<style>
.images {
    display: flex;
    max-width: 500px;
    max-height: 500px;

    overflow-x: auto;
    overflow-y: hidden;
    white-space: nowrap;
    scroll-snap-type: x mandatory;

    border-radius: 4px;

    div {
        display: block;

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
}
</style>
