<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import PieChart from "./PieChart.svelte";

  export let gj: FeatureCollection;

  let kinds = gj.features.map((f) => f.properties.kind);
</script>

<h2>Kind</h2>
<PieChart input={kinds} />
