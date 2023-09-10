<script lang="ts">
  import { CircleLayer, Popup } from "svelte-maplibre";
  import { osmUrl } from "./data";
  import Tooltip from "./Tooltip.svelte";

  export let drawHeatmap: boolean;

  function openOSM(e: CustomEvent<any>) {
    window.open(osmUrl(e.detail.features![0].properties), "_blank");
  }

  let colors = {
    hasBrand: "blue",
    local: "red",
  };
</script>

<CircleLayer
  id="input-layer"
  paint={{
    "circle-color": [
      "case",
      ["to-boolean", ["get", "brand"]],
      colors.hasBrand,
      colors.local,
    ],
    "circle-opacity": 0.8,
    "circle-radius": 10,
  }}
  layout={{ visibility: !drawHeatmap ? "visible" : "none" }}
  hoverCursor="pointer"
  on:click={openOSM}
>
  <Popup openOn="hover" let:features>
    <Tooltip props={features[0].properties} />
  </Popup>
</CircleLayer>
