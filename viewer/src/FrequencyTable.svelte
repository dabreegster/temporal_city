<script lang="ts">
  export let label: string;
  export let input: string[];

  let counts = {};
  for (let x of input) {
    counts[x] ??= 0;
    counts[x]++;
  }

  let rows = Object.entries(counts).sort((a, b) => b[1] - a[1]);
  rows.length = 10;

  // TODO https://github.com/dasDaniel/svelte-table?
</script>

<h2>{label}</h2>
<table>
  <thead> <tr><th>Value</th><th>Count</th></tr></thead>
  <tbody>
    {#each rows as [x, count]}
      <tr><td>{x}</td><td>{count.toLocaleString()}</td></tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    border: solid;
  }
</style>
