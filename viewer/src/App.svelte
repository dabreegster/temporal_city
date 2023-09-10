<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import type { Map as MapType } from "maplibre-gl";
  import { MapLibre } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import MapContents from "./MapContents.svelte";
  import Sidebar from "./Sidebar.svelte";

  let fileInput: HTMLInputElement;
  function fileLoaded(e: Event) {
    let reader = new FileReader();
    reader.onload = (e) => {
      loadFile(e.target!.result as string);
    };
    let files = fileInput.files!;
    reader.readAsText(files[0]);
  }

  let map: MapType;
  let gj: FeatureCollection | undefined;

  function loadFile(contents: string) {
    gj = JSON.parse(contents);
  }
</script>

<Layout>
  <div slot="left">
    <h1>Temporal City</h1>
    <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    {#if gj}
      {#key gj}
        <Sidebar {gj} />
      {/key}
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
    >
      {#if gj}
        <MapContents {map} {gj} />
      {/if}
    </MapLibre>
  </div>
</Layout>
