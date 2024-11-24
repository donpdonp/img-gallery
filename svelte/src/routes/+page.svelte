<script lang="ts">
  import { onMount } from "svelte";
  import { PUBLIC_IMG_API_URL, PUBLIC_IMG_STORE_URL } from "$env/static/public";

  let images = $state([]);
  let loading = $state(false);

  onMount(async () => {
    loading = true;
    let request = JSON.stringify({ start_timestamp: 0 });
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
  {#each images as image}
    <div class="image">
      <img
        src="{PUBLIC_IMG_STORE_URL}/{image.hash}?h=200"
        alt={image.filename}
      />
      {image.hash}
    </div>
  {/each}
</div>

<style>
  .image {
  }

  #images {
    display: flex;
    flex-wrap: wrap;
  }
</style>
