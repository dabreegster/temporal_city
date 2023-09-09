<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
  import type { Map as MapType } from "maplibre-gl";
  import { CircleLayer, GeoJSON, MapLibre, Popup } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import Tooltip from "./Tooltip.svelte";

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
    map.fitBounds(bbox(gj!), { padding: 100, duration: 500 });
  }

  // Suitable for passing to map.fitBounds. Work around https://github.com/Turfjs/turf/issues/1807.
  function bbox(gj: FeatureCollection): [number, number, number, number] {
    return turfBbox(gj) as [number, number, number, number];
  }
</script>

<Layout>
  <div slot="left">
    <h1>Temporal City</h1>
    <input bind:this={fileInput} on:change={fileLoaded} type="file" />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
    >
      {#if gj}
        <GeoJSON id="input" data={gj}>
          <CircleLayer
            id="input-layer"
            manageHoverState
            paint={{
              "circle-color": "red",
              "circle-radius": 5,
            }}
          >
            <Popup openOn="hover" let:features>
              <Tooltip props={features[0].properties} />
            </Popup>
          </CircleLayer>
        </GeoJSON>
      {/if}
    </MapLibre>
  </div>
</Layout>
