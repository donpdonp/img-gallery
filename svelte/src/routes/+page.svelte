<h1>Welcome to SvelteKit</h1>
<script lang="ts">
	import { onMount } from 'svelte';
	import { PUBLIC_IMG_API_URL } from '$env/static/public';

	export let data;

	onMount(async () => {
  	const pools = await fetch(PUBLIC_IMG_API_URL).then((ps) => ps.json());
	});
</script>
