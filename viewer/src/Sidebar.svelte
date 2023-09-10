<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import opening_hours from "opening_hours";
  import FrequencyTable from "./FrequencyTable.svelte";
  import TimePicker from "./TimePicker.svelte";

  export let origGj: FeatureCollection;
  export let gj: FeatureCollection;

  let date: Date | null = null;
  let includeUnknown = false;
  let numBugs = 0;

  function recalculate() {
    console.time("recalculate");
    numBugs = 0;
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
          /*console.log(
            `Bug for http://openstreetmap.org/node/${props.node_id} (${props.opening_hours}): ${err}`
          );*/
          numBugs++;
          return includeUnknown;
        }
      } else {
        return includeUnknown;
      }
    });
    gj = gj;
    console.timeEnd("recalculate");
  }
  $: recalculate(origGj, date, includeUnknown);

  $: kinds = gj.features.map((f) => f.properties!.kind);
  $: brands = gj.features.map((f) => f.properties!.brand).filter((x) => x);
</script>

<TimePicker bind:date />
<label>
  Include unknown opening hours
  <input type="checkbox" bind:checked={includeUnknown} />
</label>
<p>
  {numBugs.toLocaleString()} places excluded because of opening hours parsing bug
</p>
<FrequencyTable label="Amenity type" input={kinds} />
<FrequencyTable label="Brand" input={brands} />
