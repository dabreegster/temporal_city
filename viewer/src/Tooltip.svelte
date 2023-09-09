<script lang="ts">
  export let props: { [name: string]: any };
</script>

<table>
  <tbody>
    {#each Object.entries(props) as [key, value]}
      <tr><td>{key}</td><td>{value}</td></tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    border: solid;
  }
</style>
