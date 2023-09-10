<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
  import type { Map as MapType } from "maplibre-gl";
  import { GeoJSON, HeatmapLayer } from "svelte-maplibre";
  import DrawCircles from "./DrawCircles.svelte";

  export let map: MapType;
  export let gj: FeatureCollection;
  export let drawHeatmap: boolean;

  let once = true;
  $: if (once && gj.features.length > 0) {
    once = false;
    map.fitBounds(bbox(gj), { padding: 100, duration: 500 });
  }

  // Suitable for passing to map.fitBounds. Work around https://github.com/Turfjs/turf/issues/1807.
  function bbox(gj: FeatureCollection): [number, number, number, number] {
    return turfBbox(gj) as [number, number, number, number];
  }
</script>

<GeoJSON id="input" data={gj}>
  <HeatmapLayer
    paint={{
      "heatmap-opacity": 0.5,
    }}
    layout={{ visibility: drawHeatmap ? "visible" : "none" }}
  />
  <DrawCircles {drawHeatmap} />
</GeoJSON>
