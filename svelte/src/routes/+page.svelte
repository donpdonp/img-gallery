<script lang="ts">
  import { onMount } from "svelte";
  import { PUBLIC_IMG_API_URL, PUBLIC_IMG_STORE_URL } from "$env/static/public";
  import Time from "svelte-time";
  import * as time from "$lib/time";

  let image_groups = $state([]);
  let loading = $state(false);

  onMount(async () => {
    loading = true;
    let time_groups = time.groups(2);
    for (let group of time_groups) {
      let request = JSON.stringify({
        start_timestamp: group[0],
        stop_timestamp: group[1],
      });
      let images_resp = await fetch(PUBLIC_IMG_API_URL, {
        method: "post",
        body: request,
      }).then((ps) => ps.json());
      image_groups.push([group, images_resp.images]);
    }
    console.log($state.snapshot(image_groups));
    loading = false;
  });
</script>

<div>
  {#if loading}
    Loading images...
  {:else}{/if}
</div>

<div id="images">
  {#each image_groups as image_group}
    <div class="imagerowdate">
      <Time timestamp={image_group[0][0] * 1000} />
    </div>
    <div class="imagerow">
      {#each image_group[1] as image}
        <div class="image">
          <img
            src="{PUBLIC_IMG_STORE_URL}/{image.hash}?h=200"
            alt={image.filename}
          />
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .image {
    margin: 3px;
  }

  .imagerow {
    display: flex;
    flex-wrap: wrap;
    margin: 3px;
  }

  .imagerowdate {
    font-weight: bold;
    width: 10em;
  }
</style>
