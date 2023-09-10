<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import FrequencyTable from "./FrequencyTable.svelte";

  export let gj: FeatureCollection;

  let kinds = gj.features.map((f) => f.properties.kind);
  let brands = gj.features.map((f) => f.properties.brand).filter((x) => x);
</script>

<FrequencyTable label="Amenity type" input={kinds} />
<FrequencyTable label="Brand" input={brands} />
