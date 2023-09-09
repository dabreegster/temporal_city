<script lang="ts">
  import Plot from 'svelte-plotly.js';

  export let input: string[];

  let counts = {};
  for (let x of input) {
    counts[x] ??= 0;
    counts[x]++;
  }

  let labels = [];
  let values = [];
  for (let [x, count] of Object.entries(counts)) {
    labels.push(x);
    values.push(count);
  }
  let data = {
    labels,
    values,
    type: "pie",
  };
  console.log(data);
</script>

<Plot {data} fillParent="width" />
