<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import opening_hours from "opening_hours";
  import FrequencyTable from "./FrequencyTable.svelte";
  import TimePicker from "./TimePicker.svelte";

  export let origGj: FeatureCollection;
  export let gj: FeatureCollection;

  let date: Date | null = null;

  // TODO Happens twice onload
  $: {
    console.log(`recalcing`);
    gj.features = origGj.features.filter((f) => {
      if (date == null) {
        return true;
      }
      let props = f.properties!;
      if (props.opening_hours) {
        try {
          // TODO Disable nominatim lookup for holidays
          let oh = new opening_hours(props.opening_hours, null);
          return oh.getState(date);
        } catch (err) {
          console.log(
            `Bug for http://openstreetmap.org/node/${props.node_id} (${props.opening_hours}): ${err}`
          );
          return false;
        }
      } else {
        return false;
      }
    });
    console.log(`result is ${gj.features.length}`);
    gj = gj;
  }

  $: kinds = gj.features.map((f) => f.properties!.kind);
  $: brands = gj.features.map((f) => f.properties!.brand).filter((x) => x);
</script>

<TimePicker bind:date />
<FrequencyTable label="Amenity type" input={kinds} />
<FrequencyTable label="Brand" input={brands} />
