<script>
    export let currentProblem = {};

    const numberFormatter = new Intl.NumberFormat('en-US');

    $: potSize = currentProblem.pot_size != null ? numberFormatter.format(currentProblem.pot_size) : '...';
    $: betToCall = currentProblem.bet_to_call != null ? numberFormatter.format(currentProblem.bet_to_call) : '...';
    $: equity = currentProblem.equity != null ? currentProblem.equity.toFixed(1) : '...';
</script>

<div class="display-container">
    <div class="info-grid">
        <div class="info-box">
            <div class="label">Pot Size</div>
            <div class="value pot">{potSize}</div>
        </div>

        <div class="info-box">
            <div class="label">Bet to Call</div>
            <div class="value bet">{betToCall}</div>
        </div>

        <div class="info-box wide">
            <div class="label">Your Equity</div>
            <div class="value equity">{equity}%</div>
        </div>
    </div>
</div>

<style>
    :root {
        --gold: #d4af37;
        --blue: #3b82f6;
        --red: #ef4444;
        --dark-bg: #1e1e1e;
        --border-color: #3f3f46;
    }

    .display-container {
        background: var(--dark-bg);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 2rem;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
        max-width: 600px;
        margin: 0 auto;
    }

    .info-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1.5rem;
    }

    .info-box {
        background: #2a2a2a;
        padding: 1.5rem;
        border-radius: 12px;
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .info-box.wide {
        grid-column: 1 / -1; /* Span full width */
    }
    
    .label {
        font-size: 1rem;
        font-weight: 500;
        color: #a1a1aa;
        margin-bottom: 0.75rem;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .value {
        font-size: 2.5rem;
        font-weight: 700;
        line-height: 1.1;
    }

    .value.pot {
        color: var(--gold);
    }
    .value.bet {
        color: var(--red);
    }
    .value.equity {
        color: var(--blue);
    }
    
    @media (max-width: 480px) {
        .info-grid {
            grid-template-columns: 1fr;
        }
        .value {
            font-size: 2rem;
        }
    }
</style>