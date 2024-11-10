<script lang="ts">
	import { onMount } from 'svelte';
	import { PUBLIC_IMG_API_URL } from '$env/static/public';

  let images = $state([])

	onMount(async () => {
  	let request = JSON.stringify({start_timestamp: 0})
  	images = await fetch(PUBLIC_IMG_API_URL, {method: 'post', body: request }).then((ps) => ps.json());
	});
</script>

<h3>img-gallery</h3>
{#each images as image}
  image {JSON.stringify(image)}
{/each}
