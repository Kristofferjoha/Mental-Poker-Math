<main>
  <h1>Mental Poker Math</h1>
  <p>Sharpen your poker intuition and numerical precision.</p>

  <nav class="game-menu">
    <a href="/pot-odds-game" class="game-link">
      <h2>Pot Odds + EV decision making</h2>
      <p>Train your intuitive feel for hand strength vs. pot odds.</p>
    </a>
    
    <a href="/pure-eq-game" class="game-link">
      <h2>Pure EV</h2>
      <p>A pure math quiz on expected value.</p>
    </a>

    <a href="/pure-pot-odds-game" class="game-link">
      <h2>Pure Odds</h2>
      <p>A pure math quiz on odds and probabilities.</p>
    </a>

  </nav>
</main>

<style>
  main {
    max-width: 800px;
    margin: 2rem auto;
    text-align: center;
    font-family: system-ui, sans-serif;
  }
  .game-menu {
    margin-top: 3rem;
  }
  .game-link {
    display: block;
    background-color: #f9f9f9;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    text-decoration: none;
    color: #333;
    transition: transform 0.2s;
  }
  .game-link:hover {
    transform: scale(1.02);
    border-color: #007bff;
  }
  h2 {
    margin: 0 0 0.5rem 0;
  }
  p {
    margin: 0;
    color: #666;
  }
</style>