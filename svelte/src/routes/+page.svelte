<script lang="ts">
  import { onMount } from "svelte";
  import { browser } from "$app/environment";
  import { PUBLIC_IMG_API_URL, PUBLIC_IMG_STORE_URL } from "$env/static/public";
  import Time from "svelte-time";
  import * as time from "$lib/time";

  let image_groups = $state({ groups: [] });
  let loading = $state(false);
  let win_height = $state(0);
  let win_width = $state(0);

  onMount(async () => {
    loading = true;
    let time_groups = time.groups(4);
    image_groups.start = time_groups.start;
    image_groups.stop = time_groups.stop;
    for (let group of time_groups.groups) {
      let request = JSON.stringify({
        start_timestamp: group[0],
        stop_timestamp: group[1],
      });
      let images_resp = await fetch(PUBLIC_IMG_API_URL, {
        method: "post",
        body: request,
      }).then((ps) => ps.json());
      image_groups.groups.push([group, images_resp.images]);
    }
    console.log($state.snapshot(image_groups));
    win_width = window.innerWidth;
    win_height = window.innerHeight;
    loading = false;
  });
</script>

<div>
  {#if loading}
    Gathering images...
  {:else}
    <Time timestamp={image_groups.start * 1000} />-
    <Time timestamp={image_groups.stop * 1000} />
    (browser {win_height}x{win_width})
  {/if}
</div>

<div id="images">
  {#each image_groups.groups as image_group}
    <div>
      <span class="imagerowdate">
        <Time timestamp={image_group[0][0] * 1000} />
      </span>
      {#if image_group[1].length > 0}
        {image_group[1].length} pics
      {/if}
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
  }
</style>
