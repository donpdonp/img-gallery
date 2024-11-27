<script lang="ts">
  import { onMount } from "svelte";
  import { PUBLIC_IMG_API_URL, PUBLIC_IMG_STORE_URL } from "$env/static/public";

  let images = $state([]);
  let loading = $state(false);

  onMount(async () => {
    loading = true;
    let request = JSON.stringify({
      start_timestamp: 0,
      stop_timestamp: 2731908897,
    });
    let images_resp = await fetch(PUBLIC_IMG_API_URL, {
      method: "post",
      body: request,
    }).then((ps) => ps.json());
    images.push(...images_resp.images);
    loading = false;
  });
</script>

<h3>img-gallery</h3>

<div>
  {#if loading}
    Loading
  {:else}
    {images.length} images
  {/if}
</div>

<div id="images">
  <div class="imgdaterow">Dec 12</div>
  {#each images as image}
    <div class="image">
      <img
        src="{PUBLIC_IMG_STORE_URL}/{image.hash}?h=200"
        alt={image.filename}
      />
    </div>
  {/each}
</div>

<style>
  .image {
    margin: 3px;
  }

  #images {
    display: flex;
    flex-wrap: wrap;
  }

  .imgdaterow {
    font-weight: bold;
  }
</style>
