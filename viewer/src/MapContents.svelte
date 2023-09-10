<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
  import type { Map as MapType } from "maplibre-gl";
  import { CircleLayer, GeoJSON, Popup } from "svelte-maplibre";
  import Tooltip from "./Tooltip.svelte";

  export let map: MapType;
  export let gj: FeatureCollection;

  map.fitBounds(bbox(gj), { padding: 100, duration: 500 });

  // Suitable for passing to map.fitBounds. Work around https://github.com/Turfjs/turf/issues/1807.
  function bbox(gj: FeatureCollection): [number, number, number, number] {
    return turfBbox(gj) as [number, number, number, number];
  }

  let colors = {
    hasBrand: "blue",
    local: "red",
  };
</script>

<GeoJSON id="input" data={gj}>
  <CircleLayer
    id="input-layer"
    manageHoverState
    paint={{
      "circle-color": [
        "case",
        ["to-boolean", ["get", "brand"]],
        colors.hasBrand,
        colors.local,
      ],
      "circle-radius": 5,
    }}
  >
    <Popup openOn="hover" let:features>
      <Tooltip props={features[0].properties} />
    </Popup>
  </CircleLayer>
</GeoJSON>
