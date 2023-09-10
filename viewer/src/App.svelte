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
      origGj = JSON.parse(e.target!.result as string);
    };
    let files = fileInput.files!;
    reader.readAsText(files[0]);
  }

  let map: MapType;
  let origGj: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  // Current / filtered
  let gj: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  let drawHeatmap = true;
</script>

<Layout>
  <div slot="left">
    <h1>Temporal City</h1>
    <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    <Sidebar {origGj} bind:gj bind:drawHeatmap />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
    >
      <MapContents {map} {gj} {drawHeatmap} />
    </MapLibre>
  </div>
</Layout>
