<script>
  export let rank; // e.g., "Ace", "King", "Eight"
  export let suit; // e.g., "Spades", "Hearts", "Clubs"

  $: imagePath = `/cards/${rank}_of_${suit.toLowerCase()}.svg`;

  $: altText = `${rank} of ${suit}`;
</script>

<img src={imagePath} alt={altText} class="card-image" />

<style>
  .card-image {
    height: 100px; 
    width: auto;
    margin: 4px;

    box-shadow: 0 2px 5px rgba(0,0,0,0.2);
    border-radius: 5px;
  }
</style>