<script lang="ts">
  export let date: Date | null = null;
  let selected = "";

  let choices = [
    ["None", ""],
    ["Tuesday 12pm", new Date("September 12, 2023 12:00:00")],
    ["Tuesday 7pm", new Date("September 12, 2023 19:00:00")],
    ["Tuesday 3am", new Date("September 12, 2023 03:00:00")],
    ["Saturday 7am", new Date("September 16, 2023 07:00:00")],
    ["Saturday 12pm", new Date("September 16, 2023 12:00:00")],
    ["Saturday 11pm", new Date("September 16, 2023 21:00:00")],
  ];

  $: date = selected ? new Date(selected) : null;
</script>

<div>
  <label for="time">Time:</label>
  <select bind:value={selected} id="time">
    {#each choices as [label, date]}
      <option value={date.toString()}>{label}</option>
    {/each}
  </select>
</div>
