<script lang="ts">
  // Empty means no filter
  export let kindFilter: string = "";

  let choices = [
    ["All", ""],
    ["Fast food", "fast_food"],
  ];
</script>

<div>
  <label for="kind">Show only some category:</label>
  <select bind:value={kindFilter} id="kind">
    {#each choices as [label, value]}
      <option {value}>{label}</option>
    {/each}
  </select>
</div>
