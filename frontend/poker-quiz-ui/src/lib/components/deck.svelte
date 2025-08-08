<script>
  export let rank;
  export let suit;
</script>

<img
  src={`/cards/${rank}_of_${suit.toLowerCase()}.svg`}
  alt={`${rank} of ${suit}`}
  class="card-image"
  loading="lazy"
/>

<style>
  .card-image {
    height: 80px;
    width: 54px;
    margin: 3px;
    box-shadow: 0 2px 5px rgba(0,0,0,0.2);
    border-radius: 4px;
  }
</style>
